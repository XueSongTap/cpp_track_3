fn main() {
    let args = std::env::args();
    println!("size = {}", args.len());
    for arg in args {
        println!("{}", arg);
    }
}