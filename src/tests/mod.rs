// Apache header:
//
//     Copyright 2020 Google LLC
//
//     Licensed under the Apache License, Version 2.0 (the "License");
//     you may not use this file except in compliance with the License.
//     You may obtain a copy of the License at
//
//         https://www.apache.org/licenses/LICENSE-2.0
//
//     Unless required by applicable law or agreed to in writing, software
//     distributed under the License is distributed on an "AS IS" BASIS,
//     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//     See the License for the specific language governing permissions and
//     limitations under the License.

use super::*;
use pgn_reader::BufferedReader;
use std::io;

#[test]
fn empty_description() -> io::Result<()> {
    let pgn = b"
[White \"player1\"]
[Black \"player2\"]
";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new();
    let description = reader.read_game(&mut position_converter)?;

    assert_eq!(description, Some("".to_string()));
    Ok(())
}

#[test]
fn converts_single_exercise() -> io::Result<()> {
    let pgn = b"
[White \"player1\"]
[Black \"player2\"]
[FEN \"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2\"]
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new();
    let description = reader.read_game(&mut position_converter)?.unwrap();
    let result = "Exercise 1:
White to move:
White:
Pawn Ana2
Pawn Bela2
Pawn Caesar2
Pawn David2
Pawn Felix2
Pawn Gustav2
Pawn Hector2
Pawn Eva4
Knight Bela1
Knight Gustav1
Bishop Caesar1
Bishop Felix1
Rook Ana1
Rook Hector1
Queen David1
King Eva1
Black:
Pawn Caesar5
Pawn Ana7
Pawn Bela7
Pawn David7
Pawn Eva7
Pawn Felix7
Pawn Gustav7
Pawn Hector7
Knight Bela8
Knight Gustav8
Bishop Caesar8
Bishop Felix8
Rook Ana8
Rook Hector8
Queen David8
King Eva8
";

    assert_eq!(&description[..], result);
    Ok(())
}

#[test]
fn converts_multiple_exercises() -> io::Result<()> {
    let pgn = b"
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

 1-0

[FEN \"6qk/8/8/8/8/8/8/7K b - - 0 1\"]

 0-1
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new();
    let mut description = String::new();
    while let Some(single_exercise) = reader.read_game(&mut position_converter)? {
        description.push_str(&single_exercise);
    }

    let result = "Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Exercise 2:
Black to move:
Black:
Queen Gustav8
King Hector8
White:
King Hector1
";

    assert_eq!(&description[..], result);
    Ok(())
}

#[test]
fn parses_exercise_with_move_solutions() -> io::Result<()> {
    let pgn = b"
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

1. Kh2 Kh7 2. Rg5 Kh8

 1-0
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new();
    let description = reader.read_game(&mut position_converter)?.unwrap();

    let result = "Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Solution:
1. King Hector2 King Hector7
2. Rook Gustav5 King Hector8
";

    assert_eq!(&description[..], result);
    Ok(())
}

#[test]
fn parses_exercise_with_odd_number_of_moves() -> io::Result<()> {
    let pgn = b"
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

1. Kh2

 1-0
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new();
    let description = reader.read_game(&mut position_converter)?.unwrap();

    let result = "Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Solution:
1. King Hector2
";

    assert_eq!(&description[..], result);
    Ok(())
}

#[test]
fn parses_exercise_with_comments() -> io::Result<()> {
    let pgn = b"
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

1. Kh2 {and so the king moves}

 1-0
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new_with_config(false, true);
    let description = reader.read_game(&mut position_converter)?.unwrap();

    let result = "Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Solution:
1. King Hector2 
and so the king moves
";

    assert_eq!(&description[..], result);
    Ok(())
}

#[test]
fn parses_exercise_with_side_lines() -> io::Result<()> {
    let pgn = b"
[FEN \"7k/8/8/8/8/8/8/6RK w - - 0 1\"]

1. Kh2 (1. Kg2) Kg8 (1... Kh7 2. Kg3)

 1-0
        ";
    let mut reader = BufferedReader::new_cursor(&pgn[..]);
    let mut position_converter = PositionConverter::new_with_config(true, false);
    let description = reader.read_game(&mut position_converter)?.unwrap();

    let result = "Exercise 1:
White to move:
White:
Rook Gustav1
King Hector1
Black:
King Hector8
Solution:
1. King Hector2 
(1. King Gustav2 )
King Gustav8
(1... King Hector7
2. King Gustav3 )
";

    assert_eq!(&description[..], result);
    Ok(())
}
