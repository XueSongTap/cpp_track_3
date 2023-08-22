pub mod config;
// 文件名默认就是一个模块
// pub mod lib {
//     pub fn Hello_rutst() {
//         let my_name  = "rust";
//         let  myage = 10; // 2012 年 1 月发布
//         println!("name:{}, age:{}", my_name, myage);
//     }
// }


pub fn hello_rutst() {
    let my_name  = "rust";
    let  myage = 10; // 2012 年 1 月发布
    println!("lib1 name:{}, age:{}", my_name, myage);
}


pub mod helper {
    pub fn helper_rutst() {
        let my_name  = "rust";
        let  myage = 10; // 2012 年 1 月发布
        println!("lib1 helper name:{}, age:{}", my_name, myage);
    }
}