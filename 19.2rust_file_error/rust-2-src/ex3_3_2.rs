fn test1() {
    let mut v = vec![1,2,3];

    let first = &v[0];
    println!("v[0] = {}", first);
    
    v.push(4);
    println!("v[0] = {}", first);  // 如果这里加一行，也变成了不可用，如果没有这一行，即是在 5行结束的时候该变量就可以释放？
}

fn test2() {
    let mut v = vec![1,2,3];

    let first = &v[0];      //
    v.push(4);
    println!("v[0] = {}", first);
}
fn main() {
    test1();
    test2();
}