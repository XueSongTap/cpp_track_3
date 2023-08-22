#[derive(Debug)] //这个`derive` 属性会自动创建所需的实现，使限定的`struct` 能使用 `fmt::Debug` 打印。
pub struct UserModel { // 结构体驼峰
    pub id: i32,
    pub name: String,
    pub age: u8,
    pub tags: [&'static str; 5]
}

// 函数 蛇形
pub fn new_user_model()-> UserModel {
    UserModel{
        id: 0,
        name: String::new(),
        age: 0,
        tags: ["";5]
    }
}
// 结构体方法
impl UserModel {
    pub fn set_name(&mut self, name:String)   {
        self.name = name
    }
    pub fn set_age(&mut self, age:u8)   {
        self.age = age
    }
    pub fn get_age(&self) -> u8  {
        self.age
    }
}
