use std::cell::RefCell;
use std::thread;

fn main() {
    thread::spawn(move || {
       let c = RefCell::new(5);
       let m = c.borrow();
    
       let b = c.borrow_mut(); // thread '<unnamed>' panicked at 'already borrowed: BorrowMutError', ex3_3_3.rs:9:18
    }).join();
}