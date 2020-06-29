#[cfg(test)]
mod tests;

use pgn_reader::{Nag, Outcome, RawComment, RawHeader, SanPlus, Skip, Visitor};
use pleco::board::{piece_locations, Board};
use pleco::core::sq::SQ;
use pleco::core::File;
use pleco::core::Player;
use std;
use std::collections::HashMap;
use std::fmt::Write;
use std::str;

struct PositionConverter {
    starting_fen: String,
    final_description: String,
}

impl PositionConverter {
    fn new() -> PositionConverter {
        PositionConverter {
            starting_fen: String::from(""),
            final_description: String::from(""),
        }
    }

    fn describe_board(
        &mut self,
        board: Board,
        board_pieces: piece_locations::PieceLocations,
    ) -> String {
        let mut description = String::new();
        // Key = the FEN character that represents the piece.
        // Value = the squares containing that piece.
        let mut piece_to_squares: HashMap<char, Vec<SQ>> = HashMap::new();
        for (square, piece) in board_pieces.into_iter() {
            let piece_char = piece.character().unwrap();
            let squares = piece_to_squares.entry(piece_char).or_insert(vec![]);
            squares.push(square);
        }
        let white_pieces = "PNBRQK";
        let black_pieces = "pnbrqk";
        match board.turn() {
            Player::White => {
                write!(&mut description, "White to move:\n").unwrap();
                write!(&mut description, "White:\n").unwrap();
                self.describe_pieces(&mut description, &white_pieces, &piece_to_squares);
                write!(&mut description, "Black:\n").unwrap();
                self.describe_pieces(&mut description, &black_pieces, &piece_to_squares);
            }
            Player::Black => {
                write!(&mut description, "Black to move:\n").unwrap();
                write!(&mut description, "Black:\n").unwrap();
                self.describe_pieces(&mut description, &black_pieces, &piece_to_squares);
                write!(&mut description, "White:\n").unwrap();
                self.describe_pieces(&mut description, &white_pieces, &piece_to_squares);
            }
        };
        description
    }

    fn describe_pieces(
        &self,
        description: &mut String,
        pieces: &str,
        piece_to_squares: &HashMap<char, Vec<SQ>>,
    ) {
        for piece_with_color in pieces.chars() {
            if !piece_to_squares.contains_key(&piece_with_color) {
                continue;
            }
            // The goal here is to canonicalize the piece name, as a white queen or a black queen is still called a queen.
            // For accessing |piece_to_squares|, we still want to use |piece_with_color|, because it matters where the black vs white queen are.
            let piece = piece_with_color.to_lowercase().to_string();
            let name: &str = match &piece[..] {
                "p" => "Pawn",
                "n" => "Knight",
                "b" => "Bishop",
                "r" => "Rook",
                "q" => "Queen",
                "k" => "King",
                _ => "None",
            };
            for square in piece_to_squares[&piece_with_color].iter() {
                write!(description, "{} {}\n", name, self.describe_square(&square)).unwrap();
            }
        }
    }

    fn describe_square(&self, square: &SQ) -> String {
        let file_name: &str = match square.file() {
            File::A => "Ana",
            File::B => "Bela",
            File::C => "Caesar",
            File::D => "David",
            File::E => "Eva",
            File::F => "Felix",
            File::G => "Gustav",
            File::H => "Hector",
        };
        // Internal rank representation starts with zero, so adds one here to fix.
        let rank = square.rank_idx_of_sq() + 1;
        let mut square_description = String::new();
        write!(&mut square_description, "{}{}", file_name, rank).unwrap();
        square_description
    }
}

impl Visitor for PositionConverter {
    type Result = String;
    fn begin_game(&mut self) {}

    fn begin_headers(&mut self) {}

    fn header(&mut self, _key: &[u8], _value: RawHeader) {
        let key_str = match str::from_utf8(_key) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence in fen header key: {}", e),
        };
        if key_str == "FEN" && self.starting_fen.is_empty() {
            let fen = _value.decode_utf8().unwrap();
            self.starting_fen.push_str(&fen);
        }
    }

    fn end_headers(&mut self) -> Skip {
        Skip(false)
    }

    fn san(&mut self, _san_plus: SanPlus) {}

    fn nag(&mut self, _nag: Nag) {}

    fn comment(&mut self, _comment: RawComment) {}

    fn begin_variation(&mut self) -> Skip {
        Skip(true) // stay in the mainline
    }

    fn end_variation(&mut self) {}

    fn outcome(&mut self, _outcome: Option<Outcome>) {}

    fn end_game(&mut self) -> Self::Result {
        if !self.starting_fen.is_empty() {
            let board = Board::from_fen(&self.starting_fen[..]).unwrap();
            let board_pieces = board.get_piece_locations();
            self.final_description = self.describe_board(board, board_pieces);
        }
        // TODO: is there a way to return self.final_description directly from this mutable reference?
        // alternative 1:
        //self.final_description.clone()
        // Alternative 2:
        std::mem::replace(&mut self.final_description, String::new())
    }
}

pub fn hello_world() {
    println!("Hello, world!");
}
