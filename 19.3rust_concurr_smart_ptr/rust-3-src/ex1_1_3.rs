//error:missing lifetime specifier
//this function's return type contains a borrowed value,
//but the signature does not say whether it is borrowed from `s1` or `s2`
fn print_ret(s1: &str, s2: &str) -> &str {
    println!("s1 is {}", s1);
    s2
}
fn main() {
    let some_str: String = "Some string".to_string();
    let other_str: String = "Other string".to_string();
    let s1 = print_ret(&some_str, &other_str);
}