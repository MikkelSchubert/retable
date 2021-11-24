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
#[macro_use]
extern crate clap;
extern crate libc;
extern crate unicode_width;

mod args;

use args::*;

use std::fs::File;
use std::io::prelude::*;

use unicode_width::UnicodeWidthStr;

/// Splits a string by an optional token, or if no token is given, by any whitespace.
fn split_by<'a>(s: &'a str, token: &'a Option<String>) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    if let Some(ref c) = *token {
        Box::new(s.split(c))
    } else {
        Box::new(s.split_whitespace())
    }
}

/// Returns true if line is a comment
fn is_comment(s: &str, token: &Option<String>) -> bool {
    if let Some(token) = token {
        return s.trim_start().starts_with(token);
    }

    return false;
}

fn calculate_field_sizes(text: &str, args: &Args) -> Vec<usize> {
    let mut sizes = vec![];

    for line in text.split('\n') {
        if is_comment(line, &args.comment_token) {
            continue;
        }

        for (index, field) in split_by(line, &args.column_token).enumerate() {
            if index >= sizes.len() {
                sizes.push(0);
            }

            let len = UnicodeWidthStr::width(field);
            if sizes[index] < len {
                sizes[index] = len;
            }
        }
    }

    // Fixed padding between columns
    for value in &mut sizes {
        *value += args.width;
    }

    sizes
}

fn retable(text: &str, args: &Args) -> ::std::io::Result<()> {
    let sizes = calculate_field_sizes(text, args);
    let stdout = ::std::io::stdout();
    let mut stdout = stdout.lock();
    let mut output = String::new();

    for (item, line) in text.split('\n').enumerate() {
        if item > 0 {
            output.push('\n');
        }

        if is_comment(line, &args.comment_token) {
            output.push_str(line);
        } else {
            let mut last_len = 0;
            for (index, field) in split_by(line, &args.column_token).enumerate() {
                output.push_str(field);
                last_len = output.len();

                let len = UnicodeWidthStr::width(field);
                for _ in len..(sizes[index]) {
                    output.push(args.padding);
                }
            }

            // Remove trailing padding to create ragged rows
            output.truncate(last_len);
        }

        stdout.write_all(output.as_bytes())?;

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
            read_stdin(buffer)?;
        } else {
            File::open(filename)?.read_to_string(buffer)?;
        }
    }

    Ok(())
}

fn retable_main() -> i32 {
    let args = parse_args();
    let mut text = String::new();

    if args.filenames.is_empty() {
        if let Err(e) = read_stdin(&mut text) {
            eprintln!("Error reading from STDIN: {}", e);
            return 1;
        }
    } else if let Err(e) = read_files(&args.filenames, &mut text) {
        eprintln!("Error reading input files: {}", e);
        return 1;
    }

    if let Err(e) = retable(&text, &args) {
        // BrokenPipe is ignored, to allow use of tools like 'head'.
        if e.kind() != std::io::ErrorKind::BrokenPipe {
            eprintln!("Error retabling file: {}", e);
            return 1;
        }
    }

    0
}

fn main() {
    ::std::process::exit(retable_main());
}
