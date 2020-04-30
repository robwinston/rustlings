// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    let word = String::from("green"); // Try not changing this line :)

    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    if is_a_colour_word("blue") {
        println!("That is a colour word I know!");
    } else {
        println!("That is not a colour word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn is_a_colour_word(attempt: &str) -> bool {
    colours_i_know().iter().any(|colour| colour == attempt)
}

fn colours_i_know() -> Vec<String> {
    let mut colours = Vec::with_capacity(3);
    colours.push(String::from("green"));
    colours.push(String::from("blue"));
    colours.push(String::from("red"));
    return colours;
}