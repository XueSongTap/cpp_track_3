fn another_function() {
    println!("Hello, rust!");
}

// fn another_function(x: i32, y: i32) {
//     println!("x 的值为 : {}", x);
//     println!("y 的值为 : {}", y);
// }

// 函数名称的命名风格是小写字母以下划线分割
fn main() {
    println!("Hello, world!");
    another_function();

    // {} 可以表示为代码块，在块中可以使用函数语句，最后一个步骤是表达式，
    // 此表达式的结果值是整个表达式块所代表的值。这种表达式块叫做函数体表达式。
    // 注意：x + 1 之后没有分号，否则它将变成一条语句！
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    println!("add() = {}", add2(x,y));
}
//1、函数声明返回值类型的方式：在参数声明之后用 -> 来声明函数返回值的类型（不是 : ）
//2、Rust 不支持自动返回值类型判断！如果没有明确声明函数返回值的类型，
// 函数将被认为是"纯过程"，不允许产生返回值，return 后面不能有返回值表达式
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn add2(a: i32, b: i32) -> i32 {
    a + b +1
}