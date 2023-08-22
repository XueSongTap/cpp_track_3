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

trait Print_school {
    fn print_info(&self) {
        println!("默认trait");
        // println!("默认trait！name:{}, index:{}", self.name, self.index);
        // println!("默认trait！name:{}, index:", self.name);
    }
}

impl Print_school for Student {
}

impl Print_school for Teacher {
    fn print_info(&self) {
        println!("Teacher trait！name:{}, index:{}", self.name, self.index);
    }
}

fn main() {
    let stu = Student { name: String::from("小李"), index: 32 , is_homework_completed : false};
    let tea = Teacher {name : "隔壁老王".to_string(), index : 8, sex : String::from("男")};
    stu.print_info();
    tea.print_info();
}