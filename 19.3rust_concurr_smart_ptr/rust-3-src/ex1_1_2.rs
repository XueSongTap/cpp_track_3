//error:`y` does not live long enough
fn main() {
    let x: &Vec<i32>;
    {
        let y = Vec::new();//----+
//                               | y's lifetime
//                               |
        x = &y;//----------------|--------------+
//                               |              |
    }// <------------------------+              | x's lifetime
    println!("x's length is {}", x.len());//    |
}// <-------------------------------------------+