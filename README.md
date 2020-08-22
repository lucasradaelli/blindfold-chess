# Blindfold-chess: a tool to convert chess positions to an accessible  format
## Introduction
This tool transforms chess positions [in FEN format](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation) from a [pgn file](https://en.wikipedia.org/wiki/Portable_Game_Notation) into an accessible format which is easy to read by [screen readers](https://en.wikipedia.org/wiki/Screen_reader).

The main audience of this tool are blind chess players who are looking for to study chess games or chess tactics exercises, who can benefit by the output of this tool, which is more readable by a screen reader than hearing the FEN / pgn with a Screen Reader.

The common use case is getting a pgn file with several chess tactics exercises and their solutions, applying this tool, and studying using the resulting output.

For example:
- pgn file:
```
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

1. Kh2
```
- Output:
```
Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Solution:
1. King Hector2
```

This is still a work in progress.

This is not an officially supported Google product, and only a personal project of Lucas.

TODO: Link blog post with more info about the tool and scenarios where it is useful.

## Building
Prerequisites:
- [Install Rust and Cargo](https://www.rust-lang.org/tools/install)
- Build the program:
```shell
cargo build --release
```
- For testing changes when contributing:
```shell
cargo test
```

## Usage
```shell
./blindfold-chess <input_file.pgn> <output_file.txt>
```

Where:
- input_file.pgn: a pgn file with one or more chess games / exercises.
- output_file.txt: the path of the output file to write the converted chess games / exercises.

## Contributing
[Please see contributing page](docs/contributing.md)

## Feedback
Feedback is always welcome. Note that this is my first Rust project, if you notice ways I can be more precise in the Rust world let me know!

## Future work
- Implement side line descriptions
- Implement chess engine analysis descriptions
- Implement embeding comments from pgn files into solutions
- Implement a game only mode (currently, only exercise mode is supported)
