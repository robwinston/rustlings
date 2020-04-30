// primitive_types2.rs
// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn main() {
    // Characters (`char`)

    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!

    println!("{} is {}", 'ф', test_a_char(&'ф'));

    let test_chars = ['§', '1', 'q', 'й'];
    for test_char in test_chars.iter() {
        println!("{} is {}", test_char, test_a_char(test_char));
    }
}

fn test_a_char(your_character: &char) -> String {
    if your_character.is_alphabetic() {
        String::from("Alphabetical!")
    } else if your_character.is_numeric() {
        String::from("Numerical!")
    } else {
        String::from("Neither alphabetic nor numeric!")
    }
}
