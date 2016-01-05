/* Copyright (c) 2016 Mikkel Schubert <MSchubert@snm.ku.dk>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
extern crate argparse;
extern crate libc;
extern crate unicode_width;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use argparse::{ArgumentParser, Collect, Print, Store, StoreOption, StoreFalse};
use unicode_width::UnicodeWidthStr;


#[derive(Debug)]
struct Args {
    column_token: Option<char>,
    comment_token: Option<char>,
    padding: char,
    width: usize,
    filenames: Vec<String>,
}


macro_rules! stderr {
    ($($arg:tt)*) => ({
        let mut stderr = ::std::io::stderr();
        if let Err(msg) = writeln!(stderr, $($arg)*) {
            panic!("Error writing to STDERR: {}", msg);
        }
    })
}


/// Returns true if STDIN is a terminal.
fn is_stdin_atty() -> bool {
    unsafe { ::libc::isatty(::libc::STDIN_FILENO) != 0 }
}

/// Splits a string by an optional token, or if no token is given, by any
/// character whitespace.
fn split_by<'a>(s: &'a str, token: Option<char>) -> Box<Iterator<Item = &'a str> + 'a> {
    if let Some(c) = token {
        Box::new(s.split(c))
    } else {
        Box::new(s.split_whitespace())
    }
}


fn split_comment(line: &str, token: Option<char>) -> (&str, &str) {
    if let Some(c) = token {
        match line.find(c) {
            Some(index) => (&line[..index], &line[index..]),
            None => (&line, ""),
        }
    } else {
        (&line, "")
    }
}


fn parse_cli_char(token: &Option<String>, cli_option: &'static str) -> Option<char> {
    if let Some(ref token) = *token {
        if token.chars().count() == 1 {
            Some(token.chars().next().unwrap())
        } else {
            panic!("Token specified using {} must be a single character long!",
                   cli_option);
        }
    } else {
        None
    }
}


fn calculate_field_sizes(text: &str, args: &Args) -> Vec<usize> {
    let mut sizes = vec![];

    for line in text.split('\n') {
        let (line, _) = split_comment(&line, args.comment_token);

        for (index, field) in split_by(line, args.column_token).enumerate() {
            if index + 1 >= sizes.len() {
                sizes.push(0);
            }

            let len = UnicodeWidthStr::width(&field[..]);
            if sizes[index] < len {
                sizes[index] = len;
            }
        }
    }

    // Fixed padding between columns
    for value in sizes.iter_mut() {
        *value += args.width;
    }

    sizes
}


fn retable(text: &str, args: &Args) -> ::std::io::Result<()> {
    let sizes = calculate_field_sizes(text, args);
    let stdout = ::std::io::stdout();
    let mut stdout = stdout.lock();
    let mut output = String::new();

    for line in text.split('\n') {
        let (line, comment) = split_comment(&line, args.comment_token);

        if line.is_empty() {
            output.push_str(comment);
        } else {
            let mut last_len = 0;
            for (index, field) in split_by(line, args.column_token).enumerate() {
                output.push_str(&field);
                last_len = output.len();

                let len = UnicodeWidthStr::width(&field[..]);
                for _ in len..(sizes[index]) {
                    output.push(args.padding);
                }
            }

            if comment.is_empty() {
                // Remove trailing padding to create ragged rows
                output.truncate(last_len);
            } else {
                output.push_str(comment);
            }
        }

        output.push('\n');

        try!(stdout.write(&output.as_bytes()));

        output.clear();
    }

    Ok(())
}


fn read_stdin(buffer: &mut String) -> ::std::io::Result<()> {
    let mut stdin = ::std::io::stdin();

    if let Err(e) = stdin.read_to_string(buffer) {
        Err(e)
    } else {
        Ok(())
    }
}


fn read_files(filenames: &[String], buffer: &mut String) -> ::std::io::Result<()> {
    for filename in filenames {
        if filename == "-" {
            try!(read_stdin(buffer));
        } else {
            let mut handle = try!(File::open(filename));
            try!(handle.read_to_string(buffer));
        }
    }

    Ok(())
}


fn parse_args() -> Args {
    let mut args = Args {
        column_token: None,
        comment_token: None,
        padding: ' ',
        width: 2,
        filenames: vec![],
    };

    let mut column_token: Option<String> = None;
    let mut comment_token: Option<String> = None;
    let mut comments_enabled = true;
    let mut padding: Option<String> = None;
    let mut help: Vec<u8> = vec![];

    {
        let mut parser = ArgumentParser::new();

        parser.refer(&mut column_token)
            .add_option(&["--by"], StoreOption,
                        "Split columns using this character; \
                         defaults to any consecutive whitespace.")
            .metavar("CHAR");
        parser.refer(&mut padding)
            .add_option(&["--padding"], StoreOption,
                        "Character to use as padding; uses space by default.")
            .metavar("CHAR");
        parser.refer(&mut args.width)
            .add_option(&["--width"], Store,
                        "Number of spaces characters to add between columns. \
                         defaults to 2 characters.")
            .metavar("N");
        parser.refer(&mut comment_token)
            .add_option(&["--comment"], StoreOption,
                        "Ignore text following this character; comments are \
                         still printed, but does not influence indentation.")
            .metavar("CHAR");
        parser.refer(&mut comments_enabled)
            .add_option(&["--no-comments"], StoreFalse,
                        "If set, retable assumes that the text does not \
                         contain comments.");

        parser.refer(&mut args.filenames)
            .add_argument("filenames", Collect,
                          "Zero more input files; if no files are specified, \
                          input is read from STDIN instead.");
        parser.add_option(&["-v", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()), "Show version");

        parser.parse(env::args().collect(),
                     &mut ::std::io::stderr(),
                     &mut ::std::io::stderr())
            .map_err(|c| ::std::process::exit(c))
            .ok();

        // Save help-text for use below
        parser.print_help("retable", &mut help).unwrap();
    }

    args.column_token = parse_cli_char(&column_token, "--by");
    args.padding = parse_cli_char(&padding, "--padding").unwrap_or(' ');
    args.comment_token = if comments_enabled {
        parse_cli_char(&comment_token, "--comment").or(Some('#'))
    } else {
        None
    };

    if args.filenames.is_empty() && is_stdin_atty() {
        ::std::io::stdout().write(&help).unwrap();

        ::std::process::exit(0);
    }

    args
}


fn retable_main() -> i32 {
    let args = parse_args();
    let mut text = String::new();

    if args.filenames.is_empty() {
        if let Err(e) = read_stdin(&mut text) {
            stderr!("Error reading from STDIN: {}", e);
            return 1;
        }
    } else {
        if let Err(e) = read_files(&args.filenames, &mut text) {
            stderr!("Error reading input files: {}", e);
            return 1;
        }
    }

    if let Err(e) = retable(&text, &args) {
        // BrokenPipe is ignored, to allow use of tools like 'head'.
        if e.kind() != std::io::ErrorKind::BrokenPipe {
            stderr!("Error retabling file: {}", e);
            return 1;
        }
    }

    return 0;
}


fn main() {
    ::std::process::exit(retable_main());
}
