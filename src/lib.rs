#[cfg(test)]
mod tests;

use pgn_reader::{CastlingSide, File, Rank, Role, Square};
use pgn_reader::{Nag, Outcome, RawComment, RawHeader, San, SanPlus, Skip, Visitor};
use pleco::board::{piece_locations, Board};
use pleco::core::sq::SQ;
use pleco::core::File as ChessFile; // To avoid naming colision with pgn_reader::File.
use pleco::core::Player;
use std;
use std::collections::HashMap;
use std::fmt::Write;
use std::str;

struct PositionConverter {
    // The FEN of the position coming from the pgn header.
    starting_fen: String,
    // Holds the pgn moves, converted to an accessible-friendly version.
    moves: String,
    // Holds the final description of the game, containing initial position + |moves|.
    final_description: String,
    // Number of  the exercise coming from the pgn. A pgn file with 5 exercises would have 5 exercises for example.
    exercise_number: usize,
    // When parsing an exercise, holds the number of ply moves in the solution.
    ply_count: usize,
}

impl PositionConverter {
    fn new() -> PositionConverter {
        PositionConverter {
            starting_fen: String::from(""),
            moves: String::from(""),
            final_description: String::from(""),
            exercise_number: 0,
            ply_count: 0,
        }
    }

    fn describe_board(
        &mut self,
        board: Board,
        board_pieces: piece_locations::PieceLocations,
    ) -> String {
        let mut description = String::new();
        write!(&mut description, "Exercise {}:\n", self.exercise_number).unwrap();
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
        if !self.moves.is_empty() {
            write!(&mut description, "Solution:\n{}", self.moves).unwrap();
        }
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
            let name: &str = self.get_piece_name(&piece[..]);
            for square in piece_to_squares[&piece_with_color].iter() {
                write!(description, "{} {}\n", name, self.describe_square(&square)).unwrap();
            }
        }
    }

    fn get_piece_name(&self, piece: &str) -> &'static str {
        let piece_name = match piece {
            "p" => "Pawn",
            "n" => "Knight",
            "b" => "Bishop",
            "r" => "Rook",
            "q" => "Queen",
            "k" => "King",
            _ => "None",
        };
        piece_name
    }

    fn get_file_name(&self, file_number: u8) -> &'static str {
        let file_name = match file_number {
            0 => "Ana",
            1 => "Bela",
            2 => "Caesar",
            3 => "David",
            4 => "Eva",
            5 => "Felix",
            6 => "Gustav",
            7 => "Hector",
            _ => "unknown",
        };
        file_name
    }

    fn describe_square(&self, square: &SQ) -> String {
        let file_name: &str = self.get_file_name(square.file_idx_of_sq());
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

    fn san(&mut self, _san_plus: SanPlus) {
        self.ply_count += 1;
        // Writes the move number every two moves, E.G. 1. e4 e5 2. Nf3 Nc6.
        if self.ply_count % 2 == 1 {
            // Each two ply = one move in chess.
            let move_count: usize = self.ply_count / 2 + 1;
            write!(self.moves, "{}. ", move_count).unwrap();
        }
        match _san_plus.san {
            San::Normal {
                role,      // The piece.
                file,      // The file to disambiguate, E.G. Rfe1.
                rank,      // The rank to disambiguate, e.g. R5g7.
                capture,   // True if the piece captures another in this move.
                to,        // Destination square.
                promotion, // Contains the promotion piece if promoting.
            } => {
                if role != Role::Pawn {
                    write!(
                        self.moves,
                        "{} ",
                        self.get_piece_name(&role.char().to_string())
                    )
                    .unwrap();
                }
                if let Some(file) = file {
                    write!(self.moves, "{} ", self.get_file_name(file as u8)).unwrap();
                }
                if let Some(rank) = rank {
                    write!(self.moves, "{} ", rank.char()).unwrap();
                }
                if capture {
                    write!(self.moves, "takes ").unwrap();
                }
                write!(
                    self.moves,
                    "{}{}",
                    self.get_file_name(to.file() as u8),
                    to.rank().char()
                )
                .unwrap();
                if let Some(promotion) = promotion {
                    write!(
                        self.moves,
                        " promotes to {}",
                        self.get_piece_name(&promotion.char().to_string())
                    )
                    .unwrap();
                }
            }
            San::Castle(CastlingSide::KingSide) => write!(self.moves, "Short Castling").unwrap(),
            San::Castle(CastlingSide::QueenSide) => write!(self.moves, "Long Castling").unwrap(),
            _ => write!(self.moves, "--").unwrap(),
        }
        // Keep two moves per line.
        if self.ply_count % 2 == 0 {
            write!(self.moves, "\n").unwrap();
        } else {
            write!(self.moves, " ").unwrap();
        }
    }

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
            self.exercise_number += 1;
            self.final_description = self.describe_board(board, board_pieces);
        } else if !self.moves.is_empty() {
            // There is no exercise, but there is a regular game.
            let temp = std::mem::replace(&mut self.moves, String::new());
            std::mem::replace(&mut self.final_description, temp);
        }
        // Clears fields for next round.
        self.starting_fen.clear();
        self.moves.clear();
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
