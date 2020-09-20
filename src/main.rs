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
use clap::{App, Arg};
use pgn_reader::BufferedReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let matches = App::new("Blindfold chess")
        .version("1.0")
        .author("Lucas Radaelli <lucasradaelli@gmail.com>")
        .about("A tool to convert chess positions to an accessible format")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the output file to write converted positions")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::with_name("with_comments")
                .short("c")
                .long("with_comments")
                .help("If set, adds pgn comments into the converted positions"),
        )
        .arg(
            Arg::with_name("with_side_lines")
                .short("s")
                .long("with_side_lines")
                .help("If set, includes side lines in converted positions"),
        )
        .get_matches();

    let read_path = Path::new(matches.value_of("INPUT").unwrap());
    let input_display = read_path.display();
    let mut input_file = match File::open(&read_path) {
        Err(why) => panic!("couldn't open {}: {}", input_display, why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    let mut with_side_lines = false;
    let mut with_comments = false;
    if matches.occurrences_of("with_side_lines") > 0 {
        with_side_lines = true;
    }
    if matches.occurrences_of("with_comments") > 0 {
        with_comments = true;
    }
    let mut position_converter = PositionConverter::new_with_config(with_side_lines, with_comments);
    let mut reader = BufferedReader::new_cursor(&buffer[..]);
    let mut description = String::new();
    while let Some(single_exercise) = reader.read_game(&mut position_converter)? {
        description.push_str(&single_exercise);
    }

    let output_path = Path::new(matches.value_of("OUTPUT").unwrap());
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
