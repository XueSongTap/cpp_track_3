fn main(){
    let a =String::from("Hey !");
    let fn_closure = ||{
        println!("Closure says:{}",a);
        // let s = a;  // 加了就报错了
    };
    fn_closure();
    // fn_closure();
    println!("Main says:{}", a);
        
}
