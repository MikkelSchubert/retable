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

    Usage:
        retable [OPTIONS] [FILENAMES ...]


    positional arguments:
      filenames             Zero more input files; if no files are specified, input
                            is read from STDIN instead.

    optional arguments:
      -h,--help             show this help message and exit
      --by CHAR             Split columns using this character; defaults to tabs.
      --by-whitespace       Split columns using any consecutive whitespace;
                            defaults to tabs.
      --padding CHAR        Character to use as padding; uses space by default.
      --width N             Number of spaces characters to add between columns.
                            defaults to 2 characters.
      --comment CHAR        Ignore text following this character; comments are
                            still printed, but does not influence indentation.
      --no-comments         If set, retable assumes that the text does not contain
                            comments.
      -v,--version          Show version