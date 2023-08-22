// fn main(){
//     let b = Box::new(5);
//     println!("{}", b); // 5
// }

fn main() {
    let mut x: Box<i32> = Box::new(123);
    *x = 456;
    println!("{:?}", x);
}
