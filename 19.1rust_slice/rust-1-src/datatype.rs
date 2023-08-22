fn main() {
    // let myage:u8 = 256;     // 溢出测试
    // println!("my age :{}", myage);
    let myage = 255;  // 将值改为255、256编译
    let myage2:u8 = myage;
    println!("my age :{}", myage2);
    println!("i8最大值:{},最小值是:{}",i8::max_value() ,i8::min_value()); 
}    