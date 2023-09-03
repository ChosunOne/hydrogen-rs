use std::env;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use hydrogen::compile;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Incorrect usage. Correct usage is:");
        eprintln!("hydro input.hy");
        return ExitCode::FAILURE;
    }
    let src_file = args[1].as_str();
    match compile(src_file) {
        Ok(out_file) => {
            Command::new("nasm")
                .args(["-felf64", &out_file.to_string_lossy()])
                .output()
                .expect("Failed to create object file");

            let mut object_path =
                PathBuf::from(out_file.file_stem().unwrap().to_string_lossy().to_string());
            object_path.set_extension("o");
            let bin_out =
                PathBuf::from(out_file.file_stem().unwrap().to_string_lossy().to_string());

            let mut bin_path = env::current_dir().expect("Failed to get current dir");
            bin_path.push(bin_out);
            Command::new("ld")
                .args([
                    &object_path.to_string_lossy(),
                    "-o",
                    &out_file.file_stem().unwrap().to_string_lossy(),
                ])
                .output()
                .expect("Failed to link object file");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
