use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{execvp, fork, ForkResult};
use rustyline::Editor;
use rustyline::error::ReadlineError;
use parser::rcommand::RCommand;

pub struct Config {
    history_path: String,
    prompt: String
}

pub struct Shell<'a> {
    editor: Editor<()>,
    config: &'a Config
}

impl<'a> Shell<'a> {
    pub fn new(config: &Config) -> Shell {
        Shell {
            editor: Editor::<()>::new(),
            config: config
        }
    }

    pub fn shell_loop(&mut self) {
        if let Err(_) = self.editor.load_history(&self.config.history_path) {
            println!("No previous history.");
        }
        loop {
            match self.editor.readline(">> ") {
                Ok(line) => {
                    self.editor.add_history_entry(&line);
                    let cmd = RCommand::new(&line);
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
    }

    fn fork(&self, cmd: &RCommand) {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                let result = waitpid(child, None);
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
}
