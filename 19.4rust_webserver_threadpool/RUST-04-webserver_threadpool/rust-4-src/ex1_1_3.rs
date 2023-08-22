// use std::env;
fn main() {
    // let args = std::env::args();
    let args: Vec<String> = std::env::args().collect();
    println!("size = {}",  args.len());
    let a = &args[1];
    println!("a = {}", a);
    let b = &args[2];
    println!("b = {}", b);
}