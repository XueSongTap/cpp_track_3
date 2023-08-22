fn main() {
    println!("unicode码：");
    let ch ='汉' as i32;// unicode 码
    let ch_unicode = format!("{:X}",ch);
    println!("ch:{:?}",ch_unicode);
    println!("字节码和二进制代码：");
    let my_char = "汉".as_bytes();
    let len = my_char.len();
    for i in 0..len{
        println!("bytes[{:?}]:?],{:?}",i,&my_char[i]);
        let bit_char= format!("{:b}",my_char[i]);
        println!("i :{:?} ,bit_char :{:?}",i,bit_char);

    }
}