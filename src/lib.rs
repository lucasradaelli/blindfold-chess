#[cfg(test)]
mod tests;

use std;
use std::str;
use pgn_reader::{SanPlus, Skip, Visitor, RawHeader, RawComment,  Nag, Outcome};

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
}

impl Visitor for PositionConverter {
    type Result = String;
    fn begin_game(&mut self) {}

    fn begin_headers(&mut self) {}

    fn header(&mut self, _key: &[u8], _value: RawHeader) {
let key_str = match str::from_utf8(_key) {
    Ok(v)  => v,
    Err(e) => panic!("Invalid UTF-8 sequence in fen header key: {}", e),
};
if (key_str == "FEN" && self.starting_fen.is_empty()) {
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
        // TODO: is there a way to return self.final_description directly from this mutable reference?
        // alternative 1:
        //self.final_description.clone()
        // Alternative 2:
        self.final_description = self.starting_fen.clone();
        std::mem::replace(&mut self.final_description, String::new())
    }
}

pub fn hello_world() {
    println!("Hello, world!");
}
