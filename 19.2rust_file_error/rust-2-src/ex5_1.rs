use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();
    let key = 3;
    let value = String::from("C");

    hm.insert(1, String::from("A"));
    hm.insert(2, String::from("B"));
    hm.insert(key, value);

    println!("key = {}\n", key);
    //println!("{}", value); //错误，已经丢失了所有权

    //用循环遍历HashMap
    for iter in &hm {
        println!("{}:{}", iter.0, iter.1);
    }
    println!("");

    //用 get(key) 方法获取 value
    match hm.get(&2) {
        Some(val) => println!("2:{}\n", val),
        None => println!("No this value"),
    };

    //覆盖一个值
    hm.insert(2, String::from("b"));

    //只在map中没有对应值时才插入
    hm.entry(1).or_insert(String::from("a")); //不会插入
    hm.entry(4).or_insert(String::from("D")); //会插入

    //用循环遍历HashMap，模式匹配
    for (k,v) in &hm {
        println!("{}:{}", k, v);
    }
    println!("");

    //获取key=4对应的值的可变引用，并修改其值
    let str = hm.entry(4).or_insert(String::from("D"));
    *str = String::from("d");
    
    //用循环遍历HashMap，模式匹配
    for (k,v) in &hm {
        println!("{}:{}", k, v);
    }

    let mut map = HashMap::new();//这里没有声明散列表的泛型，是因为 Rust 的自动判断类型机制。

    map.insert("color", "red");
    map.insert("size", "10 m^2");
    map.insert("a", "aba");

    // 在已经确定有某个键的情况下如果想直接修改对应的值，有更快的办法：
    if let Some(x) = map.get_mut("a") {
        *x = "b";
    }
    //当使用 insert 方法添加新的键值对的时候，如果已经存在相同的键，会直接覆盖对应的值。如果你想"安全地插入"，就是在确认当前不存在某个键时才执行的插入动作
    map.entry("color").or_insert("red1");

    println!("映射");
    //映射表支持迭代器
    for p in map.iter() {
        println!("{:?}", p);
    }

    println!("{}", map.get("color2").unwrap()); // panicked at 'called `Option::unwrap()` on a `None` value' 在错误处理讲解
}