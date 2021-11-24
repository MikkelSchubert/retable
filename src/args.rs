use clap::{App, Arg, ArgMatches, Error, ErrorKind};

#[derive(Debug)]
pub struct Args {
    pub column_token: Option<String>,
    pub comment_token: Option<String>,
    pub padding: char,
    pub width: usize,
    pub filenames: Vec<String>,
}

pub fn parse_args() -> Args {
    let matches = App::new("retable")
        .version("0.0.2")
        .author("Mikkel Schubert")
        .arg(
            Arg::with_name("--column-token")
                .help("Split columns using this character [default: \\t]")
                .long("--column-token")
                .value_name("CHAR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("--by-whitespace")
                .help("Split columns using any consecutive whitespace.")
                .long("--by-whitespace"),
        )
        .arg(
            Arg::with_name("--padding")
                .help(
                    "Character to use as padding when printing the table \
                    [default: ' ']",
                )
                .long("--padding")
                .value_name("CHAR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("--width")
                .help("Number of spaces characters to add between columns")
                .long("--width")
                .default_value("2")
                .value_name("N")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("--comment-token")
                .help(
                    "Ignore text following this character; comments are \
                    still printed, but does not influence indentation",
                )
                .long("--comment-token")
                .value_name("CHAR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("filenames")
                .help(
                    "One or more text files to re-format. Text is read from \
                    STDIN if no files are specified and STDIN is not a terminal",
                )
                .value_name("FILE")
                .multiple(true),
        )
        .get_matches();

    let mut args = Args {
        column_token: Some(parse_string(&matches, "--column-token", "\t")),
        comment_token: matches.value_of("--comment-token").map(|v| v.to_owned()),
        padding: parse_char(&matches, "--padding", ' '),
        width: value_t!(matches.value_of("--width"), usize).unwrap_or_else(|e| e.exit()),
        filenames: parse_strings(&matches, "filenames"),
    };

    if matches.is_present("--by-whitespace") {
        args.column_token = None
    };

    if args.filenames.is_empty() && is_stdin_atty() {
        eprintln!("{}", matches.usage());
        std::process::exit(0);
    }

    args
}

fn parse_string(args: &ArgMatches, key: &str, default: &str) -> String {
    args.value_of(key).unwrap_or(default).into()
}

fn parse_strings(args: &ArgMatches, key: &str) -> Vec<String> {
    if let Some(values) = args.values_of(key) {
        values.map(|s| s.into()).collect()
    } else {
        vec![]
    }
}

fn parse_char(args: &ArgMatches, key: &str, default: char) -> char {
    if let Some(value) = args.value_of(key) {
        if value.chars().count() == 1 {
            value.chars().next().unwrap()
        } else {
            Error {
                message: format!("Expected single character, found {:?}", value),
                kind: ErrorKind::InvalidValue,
                info: Some(vec![key.to_owned()]),
            }
            .exit();
        }
    } else {
        default
    }
}

/// Returns true if STDIN is a terminal.
fn is_stdin_atty() -> bool {
    unsafe { ::libc::isatty(::libc::STDIN_FILENO) != 0 }
}
