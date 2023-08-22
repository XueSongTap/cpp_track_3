fn main() {
    // let a = 5; //不可变变量
    // a = "abc"; //错误：类型不对
    // a = 4.56;  //错误：精度不对
    // a = 456;  //错误：不可变变量
    // let mut b = 5; //可变变量
    // b = 6 //正确
    // const a = "BASE_INFO"; //常量

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x); //输出：The value of x is: 12
    let x = String::from("darren");
    println!("The value of x is: {}", x);
}