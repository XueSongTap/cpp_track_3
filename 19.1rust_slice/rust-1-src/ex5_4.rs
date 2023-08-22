// ## 5.4 结构体关联函数
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 在Rust中，函数的返回值等同于函数体最后一个表达式的值。你可以使用return关键字并指定一个值来提前从函数中返回，但大多数函数都隐式地返回了最后的表达式。
impl Rectangle {
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area(&self) -> u32 {
        self.width * self.height
        // return self.width * self.height;
    }
}

fn main() {
    let rect =  Rectangle::create(30, 50);
    println!("{:?}", rect);
    println!("rect1's area is {}", rect.area());
}