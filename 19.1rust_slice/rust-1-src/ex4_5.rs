fn main() {
    let reference_to_nothing = dangle(); // dangle()的返回的引用所对应的值是被释放了，所以不允许
}

fn dangle() -> &String {
        // 在声明以前，变量 s 无效
    let s = String::from("hello"); //
    // 这里是变量 s 的可用范围
    &s
}// 变量范围已经结束，变量 s 无效