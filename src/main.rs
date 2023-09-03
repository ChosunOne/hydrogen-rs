use std::env;
use std::fs;
use std::process::ExitCode;

use hydrogen::{split_with_char, Token};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect usage. Correct usage is:");
        eprintln!("hydro input.hy");
        return ExitCode::FAILURE;
    }
    let filename = args[1].as_str();
    let file_contents = fs::read_to_string(filename).expect("File {filename} not found");
    let split_contents = file_contents.split_whitespace().collect::<Vec<_>>();
    let split_with_semi_contents = split_with_char(&split_contents, ";");
    let tokens = split_with_semi_contents
        .into_iter()
        .map(|s| Token::try_from(s))
        .collect::<Vec<_>>();
    println!("{tokens:?}");
    ExitCode::SUCCESS
}
