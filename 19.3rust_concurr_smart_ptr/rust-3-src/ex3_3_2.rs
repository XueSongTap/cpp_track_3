use std::cell::Cell;

fn main() {
    let a = Cell::new(1);
    
    {
        let b = &a;
        b.set(2);
    }

    
    println!("{:?}", a);  // Output: 2
}