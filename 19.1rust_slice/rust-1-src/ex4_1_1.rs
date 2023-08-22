fn main(){
    let x = 5;
    let y = x;
    // println!("x:{}",x); // x:5  x5已经失效了
    println!("y:{}",y); // y:5
    
    let s1 = String::from("hello");
    let s2 = s1; 
    println!("{}, world!", s1); // 错误！s1 已经失效: [E0382] borrow of moved value: `s1`. 

    let s2 = s1.clone();        // 克隆
    println!("s1 = {}, s2 = {}", s1, s2); //s1 = hello, s2 = hello
}