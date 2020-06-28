use super::*;
use std::io;
use pgn_reader::BufferedReader;

    #[test] 
    fn empty_description()  -> io::Result<()>  {
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
    fn reads_fen()  -> io::Result<()>  {
        let pgn = b"
[White \"player1\"]
[Black \"player2\"]
[FEN \"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2\"]
        ";
        let mut reader = BufferedReader::new_cursor(&pgn[..]);
        let mut position_converter = PositionConverter::new();
        let description = reader.read_game(&mut position_converter)?;
    
        let result = "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2";
        assert_eq!(description, Some(result.to_string()));
        Ok(())
    }
