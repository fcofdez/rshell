extern crate nix;
extern crate rustyline;

mod parser;

use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
use rustyline::Editor;
use rustyline::error::ReadlineError;

fn rshell_loop() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let cmd = parser::RCommand::new(&line);
                match fork() {
                    Ok(ForkResult::Parent { child, .. }) => {
                        let result = waitpid(child, None);
                        println!("{:?}", result);
                    }
                    Ok(ForkResult::Child) => {
                        let result = execvp(&cmd.bin, &cmd.args);
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
    rl.save_history("history.txt").unwrap();
}

fn main() {
    rshell_loop();
}
