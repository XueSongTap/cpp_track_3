fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>())
}
fn main() {
    let mut Str1: String = String::from("hello Str1");
    let str1: &str = "hello str1";
    let str2 = &Str1;
    
    println!("{}", Str1.len()); // 10
    println!("{}", Str1.capacity()); // 10
    println!("{}", str1.len()); // 10
    // println!("{}", str1.capacity()); // no method named `capacity` found for reference `&str` in the current scope
}