fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>())
}
fn main() {
    let Str1: String = String::from("hello Str1");
    let str1: &str = "hello str1";
    let str2 = &Str1;
    let str3 = &Str1[0..5];
    print_type_of(&Str1); // Type is: alloc::string::String
    print_type_of(&str1); // Type is: &str
    print_type_of(&str2); // Type is: &alloc::string::String
    print_type_of(&str3); // Type is:&str
    let i = 10;
    print_type_of(&i);
}