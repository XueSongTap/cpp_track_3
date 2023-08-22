fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>())
}
fn main() {
    let str1: &str = "hello str1";
    let cover_String1 = str1.to_string();
    print_type_of(&cover_String1); // Type is: alloc::string::String

    let cover_String2 = String::from(str1);
    print_type_of(&cover_String2); // Type is: alloc::string::String

    let cover_String3 = str1.to_owned();
    print_type_of(&cover_String3); // Type is: alloc::string::String
}