use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let f = File::open("hello.txt");
    //方式1
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    //方式二
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    //如果想使一个可恢复错误按不可恢复错误处理，Result 类提供了两个办法：unwrap() 和 expect(message: &str) ：
    let f1 = File::open("hello.txt").unwrap();

    //两者的区别在于 expect 能够向 panic! 宏发送一段指定的错误信息。
    let f2 = File::open("hello.txt").expect("Failed to open.");
    

    //Result 的 Err 类型，获取 Err 类型的函数是 kind()。
    match f {
            Ok(s) => println!("{}", s),
            Err(e) => {
                match e.kind() {  // use of undeclared crate or module `io`
                    io::ErrorKind::NotFound => {
                        println!("No such file");
                    },
                    _ => {
                        println!("Cannot read the file");
                    }
                }
            }
        }

}

// //Rust 中可以在 Result 对象后添加 ? 操作符将同类的 Err 直接传递出去
// fn g(i: i32) -> Result<i32, bool> {
//     let t = f(i)?;
//     Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
// }