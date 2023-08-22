fn main() {
    let file = std::fs::File::create("data.txt").expect("create failed");
    println!("文件创建成功:{:?}",file);
 }