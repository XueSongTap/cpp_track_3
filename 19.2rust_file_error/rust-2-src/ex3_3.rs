fn main() {
    let mut v = vec![1,2,3];

    v.push(3);
    v.push(4);
    v.push(5);
    for i in v.iter() {
        println!("{}", i);
    }

    v[0] = 999;
    for i in v.iter() {
        println!("{}", i);
    }

    let first = &v[0];  //1. 使用索引语法
    println!("index v[0] = {}", first);

    /*当运行这段代码，你会发现对于第一个 [ ] 方法，当引用一个不存在的元素时 Rust 会造成 panic。
当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。当偶尔出现超过 vector 
范围的访问属于正常情况的时候可以考虑使用它。接着你的代码可以有处理 Some(&element) 或 None 的逻辑，
例如，索引可能来源于用户输入的数字。如果它们不慎输入了一个过大的数字那么程序就会得到 None 值
，你可以告诉用户当前 vector 元素的数量并再请求它们输入一个有效的值。这就比因为输入错误而使程序崩溃要友好的多！*/
    // let first = &v[12];  //1. 使用索引语法  
    // println!("index v[12] = {}", first);

    match v.get(0) {   //2. 使用 get 方法
        Some(value) => println!("get(0) = {}", value),
        None => println!("Error: No this value"), //如果请求的index不存在
    }

    match v.get(12) {   //2. 使用 get 方法
        Some(value) => println!("get[0] = {}", value),
        None => println!("Error: No this value"), //如果请求的index不存在
    }
}