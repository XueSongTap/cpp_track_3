//定义trait
pub trait GetInfo {
    fn get_name(&self) -> &String;
    fn get_index(&self) -> i32;
}

//定义学生结构体
pub struct Student {
    pub name : String,
    pub index : i32,
    Is_Homework_completed : bool
}

pub struct Teacher {
    pub name : String,
    pub index : i32,
    pub sex : String
}

//实现trait
impl GetInfo for Student {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_index(&self) -> i32 {
        self.index
    }
}

//此处我们把Teacher的实现注视掉
// impl GetInfo for Teacher {
//    fn get_name(&self) -> &String {
//        &self.name
//    }

//    fn get_index(&self) -> i32 {
//        self.index
//    }
// }

fn Print_info(item : impl GetInfo) {
    println!("name: {}", item.get_name());
    println!("index: {}", item.get_index());
}

fn main() {
    let stu = Student { name: String::from("二狗"), index: 32 , Is_Homework_completed : false};
    Print_info(stu); 
    let t = Teacher { name: String::from("小芳"), index: 5 , sex : String::from("male")};
    Print_info(t); // 能否执行
}