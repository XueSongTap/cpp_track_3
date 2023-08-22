
 
fn max<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if(x > y) {
        x
    }else {
        y
    }
}
fn get_s(s: &str) -> &str {
    s
}

fn get_a(a: &i32) -> &i32 {
    a
}

fn get_ab(a: &i32) -> &i32 {
    a
}

    
fn main() {
    let x = 10;
    let y = 20;
    let v = max(&x, &y);   
}