fn main() {
    let mut v = vec![1,2,3];

    for index in &mut v { //可变引用，遍历时可以修改其元素
        *index += 1;      //对元素进行加一操作， * 号是解引用
    }

    for index in &v {  //不可变引用，元素只读
        println!("&v{}", index);
    }

    for index in v {  // 遍历时一般不用 for index in v {} ，因为这种方式会取得 vector 的所有权。
        println!("v{}", index);
    }

    v.push(4);  // 是否会报错
}