// 1.5.1 追加 (Push)
fn str_1_push() {
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);

    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);
}
// 1.5.2 插入 (Insert)
fn str_2_insert() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
// 1.5.3 替换 (Replace)
fn str_3_replace() {
    let string_replace = String::from("I like rust. Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(string_replace);
    dbg!(new_string_replace);
}

fn str_3_replacen() {
    let string_replace = "I like rust. Learning rust is my favorite!";
    let new_string_replacen = string_replace.replacen("rust", "RUST", 1);       // 只更换1个字符串，
    dbg!(string_replace);
    dbg!(new_string_replacen);
}
fn str_3_replace_range() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
}
// 1.5.4 删除 (Delete)
fn str_4_delele_pop() {
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn str_4_delele_remove() {
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}
fn str_4_delele_truncate() {
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}
fn str_4_delele_clear() {
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}
// 1.5.5 连接 (Catenate)
fn str_5_catenate() {
    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;
    let mut result = result + "!";
    result += "!!!";

    println!("连接字符串 + -> {}", result);
}
fn str_5_catenate_format() {
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
}
fn main() {
    str_1_push();
    str_2_insert();
    str_3_replace();
    str_3_replacen();
    str_3_replace_range();
    str_4_delele_pop();
    str_4_delele_remove();
    str_4_delele_truncate();
    str_4_delele_clear();
    str_5_catenate();
    str_5_catenate_format();
}
