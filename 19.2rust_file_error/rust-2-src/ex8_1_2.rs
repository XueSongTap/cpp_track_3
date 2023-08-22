// 两个操作需要类型具有Copy语义。因此加上对类型的Copy语义要求
// PartialOrd/Copy都属于trait
fn find_max<T : PartialOrd + Copy> (list : &[T]) -> T {
    let mut max = list[0];
    for &i in list.iter() {
        if i > max {
            max = i;
        }
    }
    max
}

fn main() {
    let v_int = vec![2, 4, 1, 5, 7, 3];
    println!("max_int: {}", find_max(&v_int));
    let v_char = vec!['A', 'C', 'G', 'B', 'F'];
    println!("max_char: {}", find_max(&v_char));
}