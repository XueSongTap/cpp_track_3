fn main() {
    let mut story = String::new();
    let mut len = story.len();
    let mut capacity = story.capacity();
    println!("{}，{}", capacity, len);
    for _ in 0..5 {
        story.push_str("hello");
        len = story.len();
        capacity = story.capacity();
        println!("capacity is {} , len is {}", capacity, len);
    }
}
