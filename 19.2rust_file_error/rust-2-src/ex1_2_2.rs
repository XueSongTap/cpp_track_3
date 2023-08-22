fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>())
}
fn main() {
    let str1 = String::from("rust");
    let cover_str1 = &String::from(str1);
    print_type_of(&cover_str1); // Type is: &alloc::string::String
    
    let cover_str2_temp = String::from("hello");
    let cover_str2 = cover_str2_temp.as_str();
    print_type_of(&cover_str2); // Type is: &str
}