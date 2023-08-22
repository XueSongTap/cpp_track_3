// second_module.rs
pub fn message() -> String {
    String::from("This is the 2nd module.")
}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    fn test2() {
        let mut meal =  Breakfast::summer("Rye");       // 模块内使用可以
        let meal = Breakfast{
            toast: String::from("sanmingzhi"),
            seasonal_fruit: String::from("apple"),      // Each field should be specified exactly once 每个成员都需要进行初始化
        };
    }
}
fn test1() {
    // let mut meal = back_of_house::Breakfast::summer("Rye");  // 模块外使用需要提供对外pub
    // let meal = back_of_house::Breakfast{
    //     toast: String::from("sanmingzhi")
    //     // seasonal_fruit: String::from("sanmingzhi"),      // Each field should be specified exactly once 每个成员都需要进行初始化
    // };
}