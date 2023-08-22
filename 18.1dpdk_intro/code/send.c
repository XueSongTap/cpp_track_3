
#include <rte_eal.h>
#include <rte_ethdev.h>
#include <rte_mbuf.h>

// Platform headers
#include <linux/if_ether.h>
#include <netinet/ip.h>
#include <netinet/udp.h>

// Standard headers
#include <stdint.h>
#include <stdio.h>


#define MAKE_IPV4_ADDR(a, b, c, d) (a + (b<<8) + (c<<16) + (d<<24))

static uint32_t g_src_ip = MAKE_IPV4_ADDR(192, 168, 0, 120);
static uint32_t g_dest_ip = MAKE_IPV4_ADDR(192, 168, 0, 119);
static uint8_t g_dest_mac_addr[ETH_ALEN] = { 0x00, 0x0c, 0x29, 0x18, 0xef, 0x9d };

#define TX_RING_SIZE 1024

#define NUM_MBUFS 8191

#define MBUF_CACHE_SIZE 0

// Everyone seems to use 32. Nobody seems to know why.
#define BURST_SIZE 32

static uint8_t g_src_mac_addr[ETH_ALEN]; 

#define DPDK_QUEUE_ID_TX 0


// In DPDK, a "port" is a NIC. We will use the first NIC DPDK finds.
int g_dpdkPortId = -1;

static const struct rte_eth_conf port_conf_default = {
    .rxmode = { .max_rx_pkt_len = RTE_ETHER_MAX_LEN }
};


static void port_init(struct rte_mempool *mbuf_pool) {

    g_dpdkPortId = 0;
    while (g_dpdkPortId < RTE_MAX_ETHPORTS &&
	       rte_eth_devices[g_dpdkPortId].data->owner.id != RTE_ETH_DEV_NO_OWNER) {
		g_dpdkPortId++;
    }
    if (g_dpdkPortId == RTE_MAX_ETHPORTS) {
        rte_exit(EXIT_FAILURE, "There were no DPDK ports free.\n");
    }

	struct rte_eth_dev_info dev_info;
	struct rte_eth_conf local_port_conf = port_conf_default;
	
	rte_eth_dev_info_get(g_dpdkPortId, &dev_info);
	if (dev_info.tx_offload_capa & DEV_TX_OFFLOAD_MBUF_FAST_FREE)
		local_port_conf.txmode.offloads |= DEV_TX_OFFLOAD_MBUF_FAST_FREE;
	
 	
    // Configure the Ethernet device.
    const int num_rx_queues = 1;
    const int num_tx_queues = 1;
    //struct rte_eth_conf port_conf = port_conf_default;
    if (rte_eth_dev_configure(g_dpdkPortId, num_rx_queues, num_tx_queues, &local_port_conf)) {
        rte_exit(EXIT_FAILURE, "rte_eth_dev_configure() failed.\n");
    }

	uint16_t nb_txd = TX_RING_SIZE;
	uint16_t nb_rxd = TX_RING_SIZE;
	rte_eth_dev_adjust_nb_rx_tx_desc(g_dpdkPortId, &nb_rxd, &nb_txd);
	
	
	struct rte_eth_rxconf rxq_conf = dev_info.default_rxconf;
	rxq_conf.offloads = local_port_conf.rxmode.offloads;
	if (rte_eth_rx_queue_setup(g_dpdkPortId, 0, nb_rxd,
            rte_eth_dev_socket_id(g_dpdkPortId), &rxq_conf, mbuf_pool) < 0) {
        rte_exit(EXIT_FAILURE, "Couldn't setup RX queue.\n");
    }
	
	
	struct rte_eth_txconf txq_conf = dev_info.default_txconf;
	txq_conf.offloads = local_port_conf.txmode.offloads;
	
    // Set up TX queue.
    if (rte_eth_tx_queue_setup(g_dpdkPortId, 0, nb_txd,
            rte_eth_dev_socket_id(g_dpdkPortId), &txq_conf) < 0) {
        rte_exit(EXIT_FAILURE, "Couldn't setup TX queue.\n");
    }
	
    // Start the Ethernet port.
    if (rte_eth_dev_start(g_dpdkPortId) < 0) {
        rte_exit(EXIT_FAILURE, "Device start failed.\n");
    }

	
	rte_eth_promiscuous_enable(g_dpdkPortId);
}


static uint16_t gen_checksum(const char *buf, int num_bytes) {
    const uint16_t *half_words = (const uint16_t *)buf;
    unsigned sum = 0;
    for (int i = 0; i < num_bytes / 2; i++)
        sum += half_words[i];

    if (num_bytes & 1)
        sum += buf[num_bytes - 1];

    sum = (sum & 0xffff) + (sum >> 16);
    sum += (sum & 0xff0000) >> 16;
    sum = ~sum & 0xffff;

    return sum;
}


static void create_eth_ip_udp(uint8_t *msg, size_t total_len, uint8_t dst_mac[ETH_ALEN],
    uint32_t src_ip, uint32_t dst_ip, uint16_t udp_src_port, uint16_t udp_dst_port) {
    // Packet looks like this:
    //   Eth  |  IP  |  UDP  |  <payload>

    struct ethhdr *eth = (struct ethhdr *)msg;
    memcpy(eth->h_dest, dst_mac, ETH_ALEN);
    memcpy(eth->h_source, g_src_mac_addr, ETH_ALEN);
    eth->h_proto = htons(ETH_P_IP);

    struct iphdr *ip = (struct iphdr *)(eth + 1);
    size_t ip_len = total_len - sizeof(struct ethhdr);
    ip->ihl = 5;
    ip->version = 4;
    ip->tos = 0;
    ip->tot_len = htons((uint16_t)ip_len);
    ip->id = 0;
    ip->frag_off = 0;
    ip->ttl = 64;
    ip->protocol = IPPROTO_UDP;
    ip->check = 0;
    ip->saddr = src_ip;
    ip->daddr = dst_ip;
    ip->check = gen_checksum((char *)ip, sizeof(struct iphdr));

    struct udphdr *udp = (struct udphdr *)(ip + 1);
    size_t udp_len = ip_len - sizeof(struct iphdr);
    udp->source = htons(udp_src_port);
    udp->dest = htons(udp_dst_port);
    udp->len = htons((uint16_t)udp_len);

    // Set the UDP checksum to zero for simplicity. This is perfectly legal. It
    // just tells the the receiver not to check the checksum.
    udp->check = 0;

    // Use the packet count as the payload.
    uint32_t *payload = (uint32_t *)(udp + 1);
    static uint32_t seq_num = 0;
    *payload = htonl(seq_num++);
}


// Send the specified number of packets, using as many bursts as necessary.
static void do_send(struct rte_mempool *mbuf_pool, int num_to_send) {
    // The smallest packet allowed by Ethernet.
    const unsigned eth_total_len = 64;

    struct rte_mbuf *mbufs[BURST_SIZE];
    for (int i = 0; i < BURST_SIZE; i++) {
        mbufs[i] = rte_pktmbuf_alloc(mbuf_pool);
        if (!mbufs[i]) {
            rte_exit(EXIT_FAILURE, "Cannot alloc mbuf\n");
        }

        mbufs[i]->pkt_len = eth_total_len;
        mbufs[i]->data_len = eth_total_len;
    }

    for (int num_packets_left = num_to_send; num_packets_left > 0;) {
        int num_to_send_this_burst = BURST_SIZE;
        if (num_packets_left < BURST_SIZE) {
            num_to_send_this_burst = num_packets_left;
        }

        for (int i = 0; i < num_to_send_this_burst; i++) {
            uint8_t *packet_data = rte_pktmbuf_mtod(mbufs[i], uint8_t *);
            const int UDP_PORT = 9096;
            create_eth_ip_udp(packet_data, eth_total_len, g_dest_mac_addr,
                g_src_ip, g_dest_ip, UDP_PORT, UDP_PORT);
        }

        int num_sent = rte_eth_tx_burst(g_dpdkPortId, DPDK_QUEUE_ID_TX, mbufs, num_to_send_this_burst);

        printf("Sent %i packets\n", num_sent);
        num_packets_left -= num_sent;
    }
}


int main(int argc, char *argv[]) {
    // Initialize the Environment Abstraction Layer. All DPDK apps must do this.
    if (rte_eal_init(argc, argv) < 0) {
        rte_exit(EXIT_FAILURE, "Error with EAL initialization\n");
    }

    struct rte_mempool *mbuf_pool = rte_pktmbuf_pool_create("MBUF_POOL", NUM_MBUFS,
        MBUF_CACHE_SIZE, 0, RTE_MBUF_DEFAULT_BUF_SIZE, rte_socket_id());
    if (!mbuf_pool) {
        rte_exit(EXIT_FAILURE, "Couldn't create mbuf pool\n");
    }

    port_init(mbuf_pool);

    rte_eth_macaddr_get(g_dpdkPortId, (struct ether_addr *)g_src_mac_addr);
    printf("Our MAC: %02x %02x %02x %02x %02x %02x\n",
            g_src_mac_addr[0], g_src_mac_addr[1],
            g_src_mac_addr[2], g_src_mac_addr[3],
            g_src_mac_addr[4], g_src_mac_addr[5]);

    do_send(mbuf_pool, 1);

    return 0;
}
