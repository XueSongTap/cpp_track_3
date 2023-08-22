// 6.2 match 语法
fn main() {
    #[derive(Debug)]
    enum Book {
        Page(u32),      // 变量
        Name{url: String},
    }
    let book = Book::Page(1001);
    println!("{:?}", book); //Page(1001)

    match book {
        Book::Page(i) => {
            println!("{}", i);
        },
        Book::Name{ url } => {
            println!("{}", url);
        }
    }

    let book = Book::Name{url: String::from("darren")};
    println!("{:?}", book); //Page(1001)

    match book {
        Book::Page(i) => {
            println!("{}", i);
        },
        Book::Name{ url } => {
            println!("{}", url);
        }
    }

    // 例外情况用下划线 _ 表示：
    let t = "abc1";
    match t {
        "abc" => println!("Yes"),
        _ => {println!("other");},
    }
}