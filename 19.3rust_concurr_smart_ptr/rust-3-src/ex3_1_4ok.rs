#[derive(Debug)]
enum List {
  Node(i32, Box<List>), // 使用Box标记类型
  Nil,
}
fn main() {
    let list = List::Node(1,
    Box::new(List::Node(2,
        Box::new(List::Node(3,
            Box::new(List::Nil))
        )
    ))
    );
    println!("{:?}", list);
}
// Node(1, Node(2, Node(3, Nil)))

// // 该程序报错
// #[derive(Debug)]
// enum List {
//   Cons(i32, List), // error, 递归类型有无限的大小
//   Nil,
// }
// fn main() {
//     let list = List::Cons(1,
//         List::Cons(2,
//             List::Cons(3, List::Nil)
//         )
//     );
//     println!("{:?}", list);
// }