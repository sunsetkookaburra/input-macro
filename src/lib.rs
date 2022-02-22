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
//! ```no_run
//! use input_macro::input;
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
//!     match input!("Do you like chocolate 🍫 (y/N)? ").as_str() {
//!         "y" | "Y" => {
//!             println!("Yay! I like chocolate too 🙂.");
//!         },
//!         _ => {
//!             println!("Oh well, all the more for me 😋.");
//!         },
//!     }
//! }
//! ```

use std::fmt::Arguments;
use std::io::{self, BufRead, Write};

/// Displays formatted prompt text to the standard output and
/// then reads the next line from the standard input,
/// returning it as a [`String`].
///
/// # Panics
///
/// Panics if writing to `std::io::stdout()` fails,
/// or reading from `std::io::stdin()` fails.
///
/// # Example
/// ```no_run
/// use input_macro::input;
///
/// let name: String = input!("What's your name? ");
/// let age: i64 = input!("How old are you today {name}? ").parse().unwrap();
/// println!(
///     "In hexadecimal, thats {}{:x}!",
///     if age < 0 { "-" } else { "" }, age.abs(),
/// );
/// ```
#[macro_export]
macro_rules! input {
    () => ($crate::read_line_expect(&mut ::std::io::stdin().lock()).unwrap());
    ($($arg:tt)*) => ($crate::input_fmt(&mut ::std::io::stdin().lock(), &mut ::std::io::stdout(), format_args!($($arg)*)).unwrap());
}

/// Writes and flushes a formatted string as prompt text to the `dst` ([`Write`])
/// then reads the next line from the `src` ([`io::BufRead`]),
/// returning it as a [`io::Result<String>`].
///
/// # Errors
///
/// This function will return any I/O error reported while formatting, flushing or reading.
/// Also returns an [`io::ErrorKind::UnexpectedEof`] error if the stream reaches EOF.
///
/// # Example
/// ```
/// use std::io::Cursor;
/// use input_macro::input_fmt;
///
/// let mut source = Cursor::new("Joe Bloggs\n");
/// let mut output = Vec::new();
/// let name = input_fmt(&mut source, &mut output, format_args!("What's your {}? ", "name"));
/// assert_eq!(String::from_utf8(output).unwrap(), "What's your name? ");
/// assert_eq!(name.unwrap(), "Joe Bloggs");
/// ```
pub fn input_fmt<B: BufRead, W: Write>(
    src: &mut B,
    dst: &mut W,
    fmt: Arguments,
) -> io::Result<String> {
    dst.write_fmt(fmt)?;
    dst.flush()?;
    read_line_expect(src)
}

/// Reads the next line from `src` ([`io::BufRead`]), mapping
/// EOF to [`io::ErrorKind::UnexpectedEof`] and returning a [`io::Result<String>`].
///
/// # Errors
///
/// This function will return any I/O error reported while reading.
/// Also returns an [`io::ErrorKind::UnexpectedEof`] error if `src` reaches EOF (returns `Ok(0)`).
///
/// # Example
/// ```
/// use std::io::Cursor;
/// use input_macro::read_line_expected;
///
/// let mut source = Cursor::new("Insert Text Here\n");
/// let text = read_line_expected(&mut source);
/// assert_eq!(text.unwrap(), "Insert Text Here");
/// ```
pub fn read_line_expect<B: BufRead>(src: &mut B) -> io::Result<String> {
    src.lines().next().map_or(
        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Input BufRead reached EOF before".to_string(),
        )),
        |line| line,
    )
}
