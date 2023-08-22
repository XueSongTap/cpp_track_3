//Error:cannot return reference to local variable `i`
fn get_int_ref<'a>() -> &'a i32 {
    let i: i32 = 42;
    &i
}
fn main() {
    let j = get_int_ref();
}