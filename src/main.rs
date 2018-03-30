#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate nix;
extern crate rustyline;

mod parser;
mod shell;
mod errors;

fn main() {
    let cfg = shell::Config::new("history.txt", ">> ");
    let mut shell = shell::Shell::new(&cfg);

    if let Err(ref e) = shell.shell_loop() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
