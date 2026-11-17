use std::{env::args, process};
mod lib;

fn main() {
    let arguments : Vec<String> = args().skip(1).collect();
    
    let config = lib::Config::parse_config(&arguments).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    } );

    let mut file = lib::exe_command(&config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    
    dbg!(file);
}