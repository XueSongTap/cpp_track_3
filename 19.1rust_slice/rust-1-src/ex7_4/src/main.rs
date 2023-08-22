//因为 use 关键字把 govern 标识符导入到了当前的模块下，可以直接使用,可以使用 as 关键字为标识符添加别名。
mod nation {
    pub mod government {
        pub fn govern() {}
    }
    pub fn govern() {}
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

fn main() {
    nation_govern();
    govern();
}

// use 关键字可以与 pub 关键字配合使用：
mod nation {
    pub mod government {
        pub fn govern() {}
    }
    pub use government::govern;
}

fn main() {
    nation::govern();
}

//引用标准库
use std::f64::consts::PI;

fn main() {
    println!("{}", (PI / 2.0).sin());
}