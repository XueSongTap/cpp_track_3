 // main.rs
 mod second_module;

 fn main() {
     println!("This is the main module.");
     println!("{}", second_module::message());
 }