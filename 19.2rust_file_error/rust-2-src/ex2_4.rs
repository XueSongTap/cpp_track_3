// ex2_4.rs
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_string(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);

    let word = first_word(my_string_literal);
    
    let word = first_word(my_string);//expected `&str`, found struct `String`  help: consider borrowing here: `&my_string`
    let word = first_word(&my_string);  // 这里是正常的
    // 采用String的方式
    let word = first_word_string(&my_string[..]); //  expected struct `String`, found `str`

    let my_string_literal = "hello world";  // 类型为 &str了

    let word = first_word_string(&my_string_literal[..]); // expected struct `String`, found `str`

    let word = first_word_string(my_string_literal);       // expected struct `String`, found `str`
    let word = first_word_string(&my_string_literal);  // expected struct `String`, found `&str` found reference `&&str`
}
