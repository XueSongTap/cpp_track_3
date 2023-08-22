
struct A<T, S> {
    x : T,
    y : S
}

impl <T, S> A<T, S> {
    //通过两个不同的A来创建一个新的A
    fn crete_new_a<M, N>(self, other : A<M, N>) -> A<T, N> {
        A {
            x : self.x,
            y : other.y
        }
    }
}

fn main() {
    let a1 = A { x : 's', y : 2.2 };
    let a2 = A { x : 3.5, y : 5 };
    let a3 = a1.crete_new_a(a2);
    println!("a3.x: {}, a3.y: {}", a3.x, a3.y);
}