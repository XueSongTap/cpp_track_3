 //ex2_1.rs
 // String的切片
 fn test1(){
    let s = String::from("broadcast");

    let part1 = &s[0..5];
    let part2 = &s[5..9];

    println!("{}={}+{}", s, part1, part2);
 }
 //被部分引用，禁止更改其值
 fn test2() {
    let mut s = String::from("china");
    let slice = &s[0..3]; // 不包括3
    // s.push_str("yes!"); // 错误
    println!("slice = {}", slice);
}

// 有一个快速的办法可以将 String 转换成 &str：
fn test3(){
    let s1 = String::from("hello");
    let s2 = &s1[..];
    println!("s1:{}, s2:{}", s1, s2);
}

fn test4()
{
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}
// 切片作为函数参数
fn print_slice(s: &[i32]) {
    for i in s.iter() {
        println!("{}", i);
    }
}
// 切片作为函数参数
fn test5()
{
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    print_slice(&part);
}
fn main () {
   test1();
   test2();
   test3();
   test4();
   test5();
}
/*
fn main(){
    let mut data = [10,20,30,40,50];
    use_slice(&mut data[1..4]);
    // passes references of 
    20, 30 and 40
    println!("{:?}",data);
 }
 fn use_slice(slice:&mut [i32]) {
    println!("切片的长度为：{:?}",slice.len());
    println!("{:?}",slice);
    slice[0] = 1010; // replaces 20 with 1010
 }*/