use std::env;
use std::io::{Write, stderr};
use std::process::ExitCode;
use crate::jwt::decode_jwt;

mod jwt;

fn print_usage() {
    let _ = writeln!(stderr(), "Usage: jwtter <token>");
}

fn main() -> ExitCode {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage();
        return ExitCode::from(2)
    }
    
    match decode_jwt(&args[1]) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(stderr(), "error: {}", err);
            ExitCode::FAILURE
        }
    }
}
