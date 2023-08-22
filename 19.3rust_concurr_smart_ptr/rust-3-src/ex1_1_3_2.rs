// 注意：<'a>
// fn print_ret(s1: &str, s2: &'a str) -> &'a str { //错误的写法
fn print_ret<'a>(s1: &str, s2: &'a str) -> &'a str {
    println!("s1 is {}", s1);
    s2
}
fn main() {
    let some_str: String = "Some string".to_string();
    let other_str: String = "Other string".to_string();
    let s1 = print_ret(&some_str, &other_str);
    println!("s1:{}", s1);
}