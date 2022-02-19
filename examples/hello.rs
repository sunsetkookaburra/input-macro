use input_macro::{confirm, input};

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

    if confirm!("Do you like chocolate 🍫 (yes/no)? ") {
        println!("Yay! I like chocolate too 🙂.");
    } else {
        println!("Oh well, all the more for me 😋.");
    }
}
