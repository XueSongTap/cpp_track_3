// 定义trait及方法
trait Bird {
    fn fly(&self);
}

struct Duck;
struct Swan;

// 将trait impl到 Duck中，将重写的trait方法存入虚函数表
impl Bird for Duck {
    fn fly(&self){
        println!("duck duck");
    }
}

// 将trait impl到 Swan中，将重写的trait方法存入虚函数表
impl Bird for Swan {
    fn fly(&self){
        println!("Swan Swan");
    }
}

// 定义一个调用函数
fn print_trait_obj(p: &dyn Bird){
    p.fly();
}

fn main() {
    // 新建对象
    let duck = Duck;

    // 创建 对象的引用
    let p_duck = &duck;

    // 将对象引用 转换成 trait对象，这个过程中——trait对象为胖指针(指针1.p_duck；指针2.(虚函数表+Duck的trait方法在虚函数表中的偏移量))
    let p_bird = p_duck as &dyn Bird;

    // 当调用trait方法时，从指针1中获取对象，从指针2中获取trait方法
    // print_trait_obj(p_bird);
    p_bird.fly();  // 因为fly(&self), 所以等价于 (p_bird.vtable.fly)(p_duck)

    // 同理
    let swan = Swan;
    let p_swan = &swan;
    let p_bird = p_swan as &dyn Bird;  // 指针p_bird发生了重绑定
    p_bird.fly();
    swan.fly();
}