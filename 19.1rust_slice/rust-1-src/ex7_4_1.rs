//因为 use 关键字把 govern 标识符导入到了当前的模块下，可以直接使用,可以使用 as 关键字为标识符添加别名。
mod nation {
    pub mod government {
        pub fn govern() {
            println!("nation.government.govern");
        }
    }
    pub fn govern() {
        println!("nation.govern");
    }
}

use crate::nation::government::govern;
use crate::nation::govern as nation_govern;

fn main() {
    nation_govern();
    govern();
}