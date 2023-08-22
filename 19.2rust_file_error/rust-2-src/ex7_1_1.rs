use std::fs::File;
//我们依然可以使用match 进行处理
// 一个打开文件的例子。
fn main() {
    println!("Hello, world!");
    let p = File::open("hello.txt");
    //方式一
    // match p{
    //     Ok(_) => {println!("读取成功");},
    //     Err(e) =>{
    //             println!("读取失败，具体错误如下");
    //             println!("{}",e);
    //             println!("{:?}",e);
    //         	// 使用println!("{:?}",e); 获取更详细的错误信息。
    //     },
    // }

    //方式二
    if let Ok(file) = p {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }
}