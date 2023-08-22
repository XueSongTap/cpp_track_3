//ex2_3.rs

fn main() {
    let mut s = String::from("廖庆富");
    let liao = &s[0..3];    // 如果改成&s[0..2]
    let qing =  &s[3..];
    println!("liao:{}, qing:{}", liao, qing);
}