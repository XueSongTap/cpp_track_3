mod models;
fn main() {
    let mut user = models::user_model::new_user_model();
    println!("1 {:?}", user);
    user.id = 20;
    user.name = String::from("0voice");
    user.tags[0] = "go";
    user.tags[1] = "rust";
    println!("2 {:?}", user);

    user.set_name(String::from("king"));
    user.set_age(18);
    println!("2 {:?}, age:{}", user, user.get_age());
}
