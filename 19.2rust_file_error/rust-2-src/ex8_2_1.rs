
struct A<T> {
    x : T,
    y : T
}

impl <T> A<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}


fn main() {
    let a = A { x : 1, y : 2};
    println!("a.x: {}, a.y: {}",a.get_x(), a.get_y());
    let b = A { x : 'a', y : 's'};
    println!("b.x: {}, b.y: {}",b.get_x(), b.get_y());
}