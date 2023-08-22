fn get_str<'a>() -> &'a str {
    let s: &'static str = "I have a static lifetime.";
    s
}

// fn get_str2() -> &str {
//     let s = "I have a static lifetime.";
//     &s
// }

fn get_str3<'a>() -> &'a str {
    let s = "I have a static lifetime.";  // 等价于 let s: &'static str = "I have a static lifetime.";
    s
}

fn main() {
    println!("s:{}", get_str3());
}