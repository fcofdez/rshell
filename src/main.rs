#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;
extern crate nix;
extern crate rustyline;
#[macro_use]
extern crate nom;

mod errors;
mod parser;
mod shell;

use failure::Fail;

fn main() {
    let cfg = shell::Config::new("history.txt", ">> ");
    let mut shell = shell::Shell::new(&cfg);

    if let Err(ref err) = shell.shell_loop() {
        if let Some(bt) = err.cause().and_then(|cause| cause.backtrace()) {
            println!("{}", bt)
        }

        ::std::process::exit(1);
    }
}
