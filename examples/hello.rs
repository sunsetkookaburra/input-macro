use input_macro::input;

fn main() {
    let name = input!("What's your name? ");
    println!("Hello, {name}!");

    let age: i64 = input!("How old are you today, {name}? ").parse().unwrap();

    match age {
        i if i < 0 => {
            println!("Whoah, negative age! Impressive! 🌌");
        }
        _ => {
            println!("Happy Birthday! Congratulations! 🥳");
        }
    }

    match input!("Do you like chocolate 🍫 (y/N)? ").as_str() {
        "y" | "Y" => {
            println!("Yay! I like chocolate too 🙂.");
        }
        _ => {
            println!("Oh well, all the more for me 😋.");
        }
    }
}
