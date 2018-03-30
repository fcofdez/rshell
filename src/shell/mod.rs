use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::*;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use parser::rcommand::RCommand;

use errors::*;

pub struct Config<'a> {
    history_path: &'a str,
    prompt: &'a str,
}

pub struct Shell<'a> {
    editor: Editor<()>,
    config: &'a Config<'a>,
}

impl<'a> Config<'a> {
    pub fn new(history_path: &'a str, prompt: &'a str) -> Config<'a> {
        Config {
            history_path: history_path,
            prompt: prompt,
        }
    }
}

impl<'a> Shell<'a> {
    pub fn new(config: &'a Config) -> Shell<'a> {
        Shell {
            editor: Editor::<()>::new(),
            config: config,
        }
    }

    pub fn shell_loop(&mut self) -> Result<()> {
        self.editor
            .load_history(&self.config.history_path)
            .chain_err(|| "error loading history")?;
        loop {
            match self.editor.readline(self.config.prompt) {
                Ok(line) => {
                    self.editor.add_history_entry(&line);
                    let cmd = RCommand::new(&line);
                    self.fork(&cmd.unwrap());
                }
                Err(ReadlineError::Interrupted) => break,
                Err(ReadlineError::Eof) => break,
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        Ok(())
    }

    fn fork(&self, cmd: &RCommand) -> Result<()> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                let result = waitpid(child, None);
                println!("{:?}", result);
            }
            Ok(ForkResult::Child) => {
                if cmd.background {
                    setpgid(Pid::from_raw(0), Pid::from_raw(0));
                }
                execvp(&cmd.bin(), &cmd.cargs()).chain_err(|| "something happened in exec")?;
            }
            Err(_) => println!("Fork failed"),
        }
        Ok(())
    }
}
