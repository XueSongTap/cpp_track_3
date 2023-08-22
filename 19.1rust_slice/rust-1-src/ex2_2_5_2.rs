fn main() {
    let a = [1, 2, 3, 4, 5];
    // a 是一个长度为 5 的整型数组
    
    let b = ["January", "February", "March"];
    // b 是一个长度为 3 的字符串数组
    println!("b:{:?}", b);
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // c 是一个长度为 5 的 i32 数组
    
    let d = [3; 5];
    // 等同于 let d = [3, 3, 3, 3, 3];
    
    let first = a[0];
    let second = a[1];
    // 数组访问
    
    // a[0] = 123; // 错误：数组 a 不可变
    let mut a = [1, 2, 3];
    a[0] = 4; // 正确
    
}