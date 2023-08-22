// vector 只能储存相同类型的值，这很不方便使用。还好 Rust 中有枚举可以帮助 vector 来存储不同类型的数据。
enum Data {
    Int(i32),
    Double(f64),
    Str(String),
}
fn main() {
    let mut vec: Vec<Data> = Vec::new();
    vec.push(Data::Int(99));
    vec.push(Data::Double(99.99));
    vec.push(Data::Str(String::from("Hello Rust")));
    for index in &vec {
        match index {
            Data::Int(value) => println!("{}", value),
            Data::Double(value) => println!("{}", value),
            Data::Str(value) => println!("{}", value),
        }
    }

    let vv = vec![
        Data::Int(99),
        Data::Double(99.99),
        Data::Str(String::from("Hello Rust"))
    ];
    for index in &vv {
        match index {
            Data::Int(value) => println!("{}", value),
            Data::Double(value) => println!("{}", value),
            Data::Str(value) => println!("{}", value),
        }
    }
}