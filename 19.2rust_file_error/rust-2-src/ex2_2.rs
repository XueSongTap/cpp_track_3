//ex2_2.rs

fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let word =  &s[6..11];
    // let word =  &s[6..12]; // 如果切片越界
    println!("hello:{}, word:{}", hello, word);
}