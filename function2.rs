fn main() {
    greet_user("Martin");
}

fn greet_user(name: &str) {
    println!("Hello, {}!", name)
}