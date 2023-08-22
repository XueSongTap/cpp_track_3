
trait Triple {
    fn triple(&self) -> i32;
}

impl Triple for i32 {
    fn triple(&self) -> i32 {
        *self * 3
    }
}

fn main() {
    let a = 10;
    println!("res: {}", a.triple());
}