use std::io::Write;
fn main() {
   let mut file = std::fs::File::create("data.txt").expect("create failed");
   file.write_all("从零蛋开始教程".as_bytes()).expect("write failed");
   file.write_all("\n简单编程".as_bytes()).expect("write failed");
   println!("data written to file" );
}