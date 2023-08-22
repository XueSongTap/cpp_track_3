use std::thread;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    // let a = Arc::new(1);
    let a = Rc::new(1);
    thread::spawn(move || {
        let b = Arc::clone(&a);
        println!("{}", b);  // Output: 1
    }).join();
}