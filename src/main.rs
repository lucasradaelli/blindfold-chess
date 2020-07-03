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

extern crate blindfold_chess;
use blindfold_chess::PositionConverter;
use pgn_reader::BufferedReader;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "Usage: $./blindfold_chess <input_pgn_file> <output_file>"
    );
    let read_path = Path::new(&args[1]);
    let input_display = read_path.display();
    let mut input_file = match File::open(&read_path) {
        Err(why) => panic!("couldn't open {}: {}", input_display, why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let mut position_converter = PositionConverter::new();
    let mut reader = BufferedReader::new_cursor(&buffer[..]);
    let mut description = String::new();
    while let Some(single_exercise) = reader.read_game(&mut position_converter)? {
        description.push_str(&single_exercise);
    }

    let output_path = Path::new(&args[2]);
    let output_display = output_path.display();
    let mut output_file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_display, why),
        Ok(file) => file,
    };

    match output_file.write_all(description.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", output_display, why),
        Ok(_) => println!("successfully wrote to {}", output_display),
    }
    Ok(())
}
