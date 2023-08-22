fn main(){
    let a =String::from("Hey !");
    let fn_closure = move ||{
        println!("Closure says:{}",a);
    };
    fn_closure();
    println!("Main says:{}", a);
        
}
