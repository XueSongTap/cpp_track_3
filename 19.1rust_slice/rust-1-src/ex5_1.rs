#[derive(Debug)]
struct Site {
    domain: String,
    name: String,
    nation: String,
    found: u32
}

fn main(){
    let rust = Site {
        domain: String::from("www.rust.com"),
        name: String::from("rust"),
        nation: String::from("China"),
        found: 2013
    };
    println!("site1:{:?}", rust);
    let site = Site {
        domain: String::from("www.go.com"),
        name: String::from("go"),
        ..rust //..rust 后面不可以有逗号。这种语法不允许一成不变的复制另一个结构体实例，意思就是说至少重新设定一个字段的值才能引用其他实例的值。
    };
    // println!("site1:{:?}", rust);
    println!("site2:{:?}", site);
}