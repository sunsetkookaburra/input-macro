# input-macro

[![Repository](https://img.shields.io/static/v1?label=GitHub&message=Repository&color=blue&logo=github)](https://github.com/sunsetkookaburra/input-macro)
[![Crate Page](https://img.shields.io/crates/v/input-macro?logo=rust)](https://crates.io/crates/input-macro)
[![Documentation](https://img.shields.io/docsrs/input-macro?logo=rust)](https://docs.rs/input-macro)
[![License](https://img.shields.io/crates/l/input-macro)](https://github.com/sunsetkookaburra/input-macro/blob/main/LICENSE)

No-nonsense `input!(...)` macro for Rust.

## Example

```rust
use input_macro::input;

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

    match input!("Do you like chocolate 🍫 (y/N)? ").as_str() {
        "y" | "Y" => {
            println!("Yay! I like chocolate too 🙂.");
        },
        _ => {
            println!("Oh well, all the more for me 😋.");
        },
    }
}
```

## License

MPL v2.0, see [LICENSE](./LICENSE)
