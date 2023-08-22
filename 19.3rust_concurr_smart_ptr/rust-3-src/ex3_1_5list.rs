//复习
// pub enum Option<T> {
//     None,
//     Some(T),
// }
// Option主要有以下一些用法：
// 初始化值；
// 作为在整个输入范围内没有定义的函数的返回值；
// 作为返回值，用None表示出现的简单错误；
// 作为结构体的可选字段；
// 作为结构体中可借出或者是可载入的字段；
// 作为函数的可选参数；
// 代表空指针；
// 用作复杂情况的返回值。

// 定义节点
#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>, // 节点的next指向下一个节点，可能为空，故类型为Option； 避免编译器无法计算节点大小，用Box包裹Node；
}
 
impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, next: None })
    }
 
    fn link(&mut self, node: Box<Node>) {
        self.next = Some(node);
    }
}
// 定义链表结构体
#[derive(Debug)]
struct BoxedLinkedList {
    head: Option<Box<Node>>,
}
 
impl BoxedLinkedList {
    fn new() -> BoxedLinkedList {
        BoxedLinkedList { head: None }
    }
 
    fn display(&self) {
        println!("{:?}", self);
    }
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    /*
    将list.head 这个Option包含的值用前面学到的take()方法取出；
    然后link到新节点后面；
    头指针指向新的节点；
    */
    fn push_front(&mut self, val: i32) {
        let mut new_node = Node::new(val);
        if let Some(node) = self.head.take() {
            new_node.link(node);
        }
        self.head = Some(new_node);
    }

    fn pop_front(&mut self) -> Option<i32> {
        // 获取头指针的包裹的值；生成一个新的Option；
        match self.head.take() {
            None => None, // 为空的话直接返回None；
            Some(node) => {
                // 注意此时拥有 node 的所有权；
                self.head = node.next; // Box自动解引用，然后将node的next的所有权转移给变量head；
                Some(node.val)
            }
        }
    }
}

fn main() {
    let mut list = BoxedLinkedList::new();
    for i in 1..3 {
        list.push_front(i);
    }
 
    list.display();

    println!();
 
    while !list.is_empty() {
        let v = list.pop_front().unwrap(); // unwrap 正常是返回对应的正常值，异常是直接panic
        println!("pop '{}' from head of list", v);
        print!("now list is : ");
        list.display();
    }
}