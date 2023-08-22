fn main(){
    let mut a =String::from("Hey !");
    let mut fn_closure = ||{
        a.push_str(" Alice");
    };
    fn_closure();
    println!("Main says:{}", a);
        
}
