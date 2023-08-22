//不使用泛型
//针对于整型数据
fn findmax_int(list : &[i32]) -> i32 {
    let mut max_int = list[0];
    for &i in list.iter() {
        if i > max_int {
            max_int = i;
        }
    }
    max_int
}

//针对于char数据
fn findmax_char(list : &[char]) -> char {
    let mut max_char = list[0];
    for &i in list.iter() {
        if i > max_char {
            max_char = i;
        }
    }
    max_char
}

fn main() {
    let v_int = vec![2, 4, 1, 5, 7, 3];
    println!("max_int: {}", findmax_int(&v_int));
    let v_char = vec!['A', 'C', 'G', 'B', 'F'];
    println!("max_char: {}", findmax_char(&v_char));
}