*****************************************
Retable - pretty-printing of tabular data
*****************************************

The 'retable' command is a simple pretty-printer for tabular data, similar to the 'column -t' command, but includes handling of embedded comments, and wide unicode characters.


Installation
------------

Retable is implemented in the `Rust programming language <https://www.rust-lang.org>`_, and can be compiled using 'cargo' once Rust has been installed::

    $ git clone https://github.com/MikkelSchubert/retable.git
    $ cd retable
    $ cargo build --release

The resulting binary is located in the 'target/release' folder.

If Rust v1.5 or later is installed, the above steps can be performed using the 'cargo install' command:

    $ cargo install --git https://github.com/MikkelSchubert/retable.git

Note that you may need to add ~/.cargo/bin to your PATH::

    $ echo 'export PATH=$PATH:~/.cargo/bin' >> ~/.bashrc
    $ source ~/.bashrc


Usage
-----

::

    USAGE:
        retable [FLAGS] [OPTIONS] [FILE]...

    FLAGS:
            --by-whitespace    Split columns using any consecutive whitespace
        -h, --help             Prints help information
        -V, --version          Prints version information

    OPTIONS:
            --column-token <CHAR>     Split columns using this character [default: \t]
            --comment-token <CHAR>    Ignore text following this character; comments are
                                      still printed, but does not influence indentation
            --padding <CHAR>          Character to use as padding when printing the
                                      table [default: ' ']
            --width <N>               Number of spaces characters to add between columns
                                      [default: 2]

    ARGS:
        <FILE>...    One or more text files to re-format. Text is read from STDIN if no
                     files are specified and STDIN is not a terminal
