mod second;
use second::ClassName;

fn main() {
    let object = ClassName::new(1024);
    object.public_method();
}