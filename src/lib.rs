/*
 * Copyright 2022 - Oliver Lenehan (sunsetkookaburra)
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! # input-macro - No-nonsense input!(...) macro for Rust.
//!
//! # Example
//!
//! ```
//! use input_macro::{confirm, input};
//!
//! fn main() {
//!     let name = input!("What's your name? ");
//!     println!("Hello, {name}!");
//!
//!     let age: i64 = input!("How old are you today, {name}? ").parse().unwrap();
//!
//!     match age {
//!         i if i < 0 => {
//!             println!("Whoah, negative age! Impressive! 🌌");
//!         },
//!         _ => {
//!             println!("Happy Birthday! Congratulations! 🥳");
//!         },
//!     }
//!
//!     if confirm!("Do you like chocolate 🍫 (yes/no)? ") {
//!         println!("Yay! I like chocolate too 🙂.");
//!     } else {
//!         println!("Oh well, all the more for me 😋.");
//!     }
//! }
//! ```

use std::fmt::Arguments;
use std::io::{self, stdin, stdout, Write};

/// Reads the next available line (without CR/CRLF) from the standard input.
pub fn next_line() -> io::Result<String> {
    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();
    line.truncate(line.trim_end_matches(['\r', '\n']).len());
    Ok(line)
}

fn _answer(s: &str) -> Option<bool> {
    match s {
        "y" | "yes" => Some(true),
        "n" | "no" => Some(false),
        #[cfg(feature = "emoji")]
        "👍" | "👌" | "🆗" => Some(true),
        #[cfg(feature = "emoji")]
        "👎" | "🆖" => Some(false),
        #[cfg(feature = "alias")]
        "yea" | "aye" | "yeah" | "good" | "yay" | "ye" | "true" | "do" | "go" | "sure" | "yep"
        | "always" | "positive" => Some(true),
        #[cfg(feature = "alias")]
        "nah" | "na" | "nay" | "nope" | "false" | "don't" | "dont" | "stop" | "nup" | "bad"
        | "never" | "not" | "nop" | "negative" => Some(false),
        _ => None,
    }
}

/// Return whether `s` is a 'yes', 'no', or 'other' answer.
pub fn answer(s: &str) -> Option<bool> {
    _answer(s.to_ascii_lowercase().as_str())
}

#[doc(hidden)]
pub fn _input(fmt: Arguments) -> io::Result<String> {
    stdout().write_fmt(fmt)?;
    stdout().flush()?;
    next_line()
}

fn _confirm_once(fmt: Arguments) -> io::Result<Option<bool>> {
    Ok({
        let mut line = _input(fmt)?;
        line.make_ascii_lowercase();
        _answer(&line)
    })
}

#[doc(hidden)]
pub fn _confirm(fmt: Arguments) -> io::Result<bool> {
    Ok(loop {
        match _confirm_once(fmt)? {
            Some(v) => break v,
            None => {}
        }
    })
}

/// Attempts to display the formatted prompt to the standard output
/// then read the next line (CR or CRLF) from the standard input.
/// Returns [`io::Result<String>`] (see [`input!`] for more).
///
/// # Examples
/// ```no_run
/// println!("Hello, {}!", try_input!("What's your name? ").unwrap());
/// ```
#[macro_export]
macro_rules! try_input {
    () => ($crate::next_line());
    ($($arg:tt)*) => ($crate::_input(format_args!($($arg)*)));
}

/// Attempts to display the formatted prompt to the standard output
/// then reads lines (CR or CRLF) from the standard input,
/// until either a 'yes' or a 'no' answer is recorded.
/// Returns [`io::Result<bool>`] (see [`confirm!`] for more).
///
/// # Example
///
/// ```
/// let answer: bool = try_confirm!("Do you like chocolate 🍫 (yes/no)? ").unwrap();
/// ```
#[macro_export]
macro_rules! try_confirm {
    () => ($crate::_confirm(format!("(yes/no) ")));
    ($($arg:tt)*) => ($crate::_confirm(format_args!($($arg)*)));
}

/// Displays the formatted prompt to the standard output
/// then reads the next line (CR or CRLF) from the standard input,
/// and returns it as a [`String`].
///
/// # Panics
///
/// Panics if writing to `std::io::stdout()` fails,
/// or reading from `std::io::stdin()` fails.
///
/// # Examples
/// ```no_run
/// let name: String = input!("What's your name? ");
/// let age: i64 = input!("How old are you today {name}? ").parse().unwrap();
/// println!(
///     "In hexadecimal, thats {}{:x}!",
///     if age < 0 { "-" } else { "" }, age.abs(),
/// );
/// ```
#[macro_export]
macro_rules! input {
    () => ($crate::try_input!().unwrap());
    ($($arg:tt)*) => ($crate::try_input!($($arg)*).unwrap());
}

/// Displays the formatted prompt to the standard output
/// then reads lines (CR or CRLF) from the standard input,
/// until either a 'yes' or a 'no' answer is recorded.
/// Returns [`bool`].
///
/// # Panics
///
/// Panics if writing to `std::io::stdout()` fails,
/// or reading from `std::io::stdin()` fails.
///
/// # Example
///
/// ```
/// // Output:
/// // ... Do you like chocolate 🍫 (yes/no)? hello
/// // ... Do you like chocolate 🍫 (yes/no)? yes
/// // ... Yay! I like chocolate too 🙂.
///
/// if confirm!("Do you like chocolate 🍫 (yes/no)? ") {
///     println!("Yay! I like chocolate too 🙂.");
/// }
/// else {
///     println!("Oh well, all the more for me 😋!");
/// }
/// ```
#[macro_export]
macro_rules! confirm {
    () => ($crate::try_confirm!().unwrap());
    ($($arg:tt)*) => ($crate::try_confirm!($($arg)*).unwrap());
}
