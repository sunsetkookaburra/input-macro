use input_macro::{input, input_fmt, read_line_expect};
use std::io::{self, BufRead, Cursor};

#[cfg(test)]
#[allow(unused)]
fn input_macro_usage() {
    input!();
    input!("ABC");
    input!("ABC {}", 123);
}

fn input_fmt_generic<B: BufRead>(src: &mut B) -> io::Result<String> {
    let mut output = Vec::new();
    let text = input_fmt(src, &mut output, format_args!("Prompt: "));
    assert_eq!(String::from_utf8(output).unwrap(), "Prompt: ");
    text
}

#[test]
fn input_fmt_lf() {
    let text = input_fmt_generic(&mut Cursor::new("Insert Text Here\n"));
    assert_eq!(text.unwrap(), "Insert Text Here");
}

#[test]
fn input_fmt_crlf() {
    let text = input_fmt_generic(&mut Cursor::new("Insert Text Here\r\n"));
    assert_eq!(text.unwrap(), "Insert Text Here");
}

#[test]
fn input_fmt_final() {
    let text = input_fmt_generic(&mut Cursor::new("Insert Text Here"));
    assert_eq!(text.unwrap(), "Insert Text Here");
}

#[test]
fn read_line_expect_typical() {
    let text = read_line_expect(&mut Cursor::new("Insert Text Here\n"));
    assert_eq!(text.unwrap(), "Insert Text Here");
}

#[test]
fn read_line_expect_eof() {
    let text = read_line_expect(&mut Cursor::new(""));
    assert_eq!(text.unwrap_err().kind(), io::ErrorKind::UnexpectedEof);
}
