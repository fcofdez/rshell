#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate nix;
extern crate rustyline;

use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{execvp, fork, ForkResult};
use rustyline::Editor;
use rustyline::error::ReadlineError;

mod parser;
mod shell;
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

fn rshell_loop() -> Result<()> {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let cmd = parser::rcommand::RCommand::new(&line);
                match fork() {
                    Ok(ForkResult::Parent { child, .. }) => {
                        let result = waitpid(child, Some(WaitPidFlag::WUNTRACED));
                        println!("{:?}", result);
                    }
                    Ok(ForkResult::Child) => {
                        let result = execvp(&cmd.bin(), &cmd.cargs());
                        println!("{:?}", result);
                        ();
                    }
                    Err(_) => println!("Fork failed"),
                }
            }
            Err(ReadlineError::Interrupted) => break,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
        .chain_err(|| "unable to save history.txt file")?;
    Ok(())
}

fn main() {
    if let Err(ref e) = rshell_loop() {
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
