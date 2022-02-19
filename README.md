# input-macro

![Repository](https://img.shields.io/static/v1?label=GitHub&message=Repository&color=blue&logo=github)
![Crate Page](https://img.shields.io/crates/v/input-macro?logo=rust)
![Documentation](https://img.shields.io/docsrs/input-macro?logo=rust)
![License](https://img.shields.io/crates/l/input-macro)

Simple no-nonsense `input!(...)` macro for Rust.

+ [📦 Crate Page](https://crates.io/crates/input-macro)
+ [📚 Documentation](https://docs.rs/input-macro)
+ [💻 Repository](https://github.com/sunsetkookaburra/input-macro)
+ [💻 License](https://github.com/sunsetkookaburra/input-macro/blob/main/LICENSE)

## Example

```rust
use input_macro::{confirm, input};

fn main() {
    let name = input!("What's your name? ");
    println!("Hello, {name}!");

    let age: i64 = input!("How old are you today, {name}? ").parse().unwrap();

    match age {
        i if i < 0 => {
            println!("Whoah, negative age! Impressive! 🌌");
        },
        _ => {
            println!("Happy Birthday! Congratulations! 🥳");
        },
    }

    if confirm!("Do you like chocolate 🍫 (yes/no)? ") {
        println!("Yay! I like chocolate too 🙂.");
    } else {
        println!("Oh well, all the more for me 😋.");
    }
}
```

## License

MPL v2.0, see [LICENSE](./LICENSE)
