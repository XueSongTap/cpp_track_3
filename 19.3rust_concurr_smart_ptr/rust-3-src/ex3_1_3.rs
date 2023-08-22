// 该程序报错
#[derive(Debug)]
enum List {
  Cons(i32, List), // error, 递归类型有无限的大小
  Nil,
}
fn main() {
    let list = List::Cons(1,
        List::Cons(2,
            List::Cons(3, List::Nil)
        )
    );
    println!("{:?}", list);
}
