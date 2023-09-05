mod generator;
mod parser;
mod tokenizer;

use parser::{parse_program, ParseError};
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;
use thiserror::Error;
use tokenizer::{tokenize_source, TokenError};

use generator::GeneratorError;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("CompileError: {0}")]
    File(#[from] std::io::Error),
    #[error("CompileError: {0}")]
    Token(#[from] TokenError),
    #[error("CompileError: {0}")]
    Parse(#[from] ParseError),
    #[error("CompileError: {0}")]
    Generate(#[from] GeneratorError),
}

pub fn compile(source_file: &str) -> Result<PathBuf, CompileError> {
    let file_contents = fs::read_to_string(source_file)?;
    let tokens = tokenize_source(&file_contents)?;
    let tree = parse_program(&mut tokens.iter())?;
    let generated_code = tree.generate()?;

    let out_file_path = PathBuf::from(source_file)
        .file_stem()
        .map(|s| s.to_owned())
        .unwrap_or(std::env::current_dir()?.into_os_string());

    let mut out_file = PathBuf::from(out_file_path);
    out_file.set_extension("asm");

    let mut out = fs::File::create(out_file.clone())?;
    out.write_all(generated_code.as_bytes())?;

    Ok(out_file)
}
