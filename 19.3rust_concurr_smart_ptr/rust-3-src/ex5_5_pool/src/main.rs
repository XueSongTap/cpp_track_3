use std::time::Duration;
use std::sync::{Arc, Mutex};
mod lib;


fn main() {
    let mut tasks = vec![];

    let counter = Arc::new(Mutex::new(0));
    let total = 200;
    for i in 0..total {
        let counter1 = Arc::clone(&counter);
        tasks.push(move || {
            std::thread::sleep(Duration::from_millis(i));
            let mut num = counter1.lock().unwrap();
            *num += 1;
            println!("num:{}", num);
        });
    }

    let pool = lib::ThreadPool::new(10).unwrap();   // 创建线程池

    for task in tasks {
        pool.execute(task);  //入队列
    }

    // guarantee to call ThreadPool::drop before check
    std::mem::drop(pool);
}
