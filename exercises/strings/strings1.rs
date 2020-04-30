// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

fn main() {
    println!("My current favorite color is {}", current_favorite_color());
    println!("My second favorite color is {}", second_favorite_color());
}

fn current_favorite_color() -> String {
    String::from("blue")
}

fn second_favorite_color() -> String {
    "yellow".to_string()
}
