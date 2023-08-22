//定义trait
pub trait GetInfo {
    fn get_name(&self) -> &String;
    fn get_index(&self) -> i32;
}

//定义学生结构体
pub struct Student {
    pub name : String,
    pub index : i32,
    is_homework_completed : bool
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

impl GetInfo for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_index(&self) -> i32 {
        self.index
    }
}

fn main() {
    let stu = Student { name: String::from("二狗"), index: 32 , is_homework_completed : false};
    println!("stu: {}, {}", stu.get_name(), stu.get_index());
    let t = Teacher { name: String::from("小芳"), index: 5 , sex : String::from("male")};
    println!("t: {}, {}", t.get_name(), t.get_index());
}