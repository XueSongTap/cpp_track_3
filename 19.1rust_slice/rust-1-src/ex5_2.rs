
fn main(){
    #[derive(Debug)]
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);
    
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    println!("black is {:?}", black); //引入库：[derive(Debug)]，可以用 {:?} 占位符输出一整个结构体
    println!("origin is {:#?}", black); //引入库：[derive(Debug)]，属性较多的话可以使用另一个占位符 {:#?}

}