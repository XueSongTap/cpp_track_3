// 6.1 枚举基本语法

fn test1() {
    #[derive(Debug)]
    enum Book {
        Page, Name
    }
    let book = Book::Page;
    println!("{:?}", book); //Page
}

fn test2() {
    #[derive(Debug)]
    enum Book {
        Page(u32),      // 变量
        Name(String),
    }
    let book = Book::Page(1001);
    println!("{:?}", book); //Page(1001)
}

fn test3() {
    #[derive(Debug)]
    enum Book {
        Page{index: u32},       // 结构体类型
        Name{url:String},
    }
    let book = Book::Page{index: 1001};
    println!("{:?}", book); //Page{ index: 1001 }
}
fn main() {
    test1();
    test2();
    test3();
}