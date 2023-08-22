// use 关键字可以与 pub 关键字配合使用：
mod nation {
    pub mod government {
        pub fn govern() {
            println!("nation.government.govern");
        }
    }
    pub use  government::govern;
    // pub use self::government::govern;
}

fn main() {
    nation::govern();
}