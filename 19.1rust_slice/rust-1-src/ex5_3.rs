// 5.3 结构体方法
struct Rectangle {
    width: u32,
    height: u32,
}
// 在Rust中，函数的返回值等同于函数体最后一个表达式的值。你可以使用return关键字并指定一个值来提前从函数中返回，但大多数函数都隐式地返回了最后的表达式。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
        // return self.width * self.height;
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}