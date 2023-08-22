fn main() {
    if true {
        println!("haha")
    } else {
        println!("hehe")
    }
    let a = 1;
    let number = if a > 0 { 1 } else { -1 };
     //  for 循环使用三元语句控制循环，但是 Rust 中没有这种用法
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("for i:{}", i);
    } 

    let s = ['R', 'U', 'S', 'T', '!'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'T' {
            break i;
        }
        i += 1;
    };
    println!("loop \'T\' 的索引为 {}", location); //'T' 的索引为 3

    i = 0;
    while i < s.len() {
        let ch = s[i];
        if ch == 'T' {
            break;
        }
        i += 1;
    }
    println!("while \'T\' 的索引为 {}", i);//'T' 的索引为 3

    for n in 0..s.len() {
        let ch = s[n];
        if ch == 'T' {
            i = n;
            break;
        }
    }
    println!("for \'T\' 的索引为 {}", i);//'T' 的索引为 3
}