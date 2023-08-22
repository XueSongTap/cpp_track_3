fn min<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x < y {
        x
    } else {
        y
    }
}
fn main() {
    let p = 42;
    {
        let q = 10;
        let r = min(&p, &q);
        println!("Min is {}", r);
    }
}
// fn get_int_ref<'a>() -> &'a i32 {
//     let i: i32 = 42;
//     &i
// }
// fn main() {
//     let j = get_int_ref();
// }