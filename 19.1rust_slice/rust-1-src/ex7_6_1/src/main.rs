// main.rs
// mod lib; // 默认加载lib.rs或者  lib/mod.rs
mod lib1;
mod lib2;
fn main() {
    lib1::config::show_version();
    lib1::hello_rutst();
    lib1::helper::helper_rutst();
    lib2::hello_rutst();
    lib2::helper::helper_rutst();
}
