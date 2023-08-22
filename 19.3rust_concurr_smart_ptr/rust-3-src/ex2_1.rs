fn main() {
    let add = |a, b| {
        a + b
    };
    add(1, 2); // 3
    add(1, 2.1); // 在第五行已经被自动推导过
}