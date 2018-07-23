use nix::sys::wait::waitpid;
use nix::unistd::*;
use parser::rcommand::RCommand;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use errors::Failures;

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

    pub fn shell_loop(&mut self) -> Result<(), Failures> {
        self.editor.load_history(&self.config.history_path)?;
        loop {
            match self.editor.readline(self.config.prompt) {
                Ok(line) => {
                    self.editor.add_history_entry(&line);
                    let cmd = RCommand::new(&line);
                    self.fork(&cmd.unwrap())?;
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

    fn fork(&self, cmd: &RCommand) -> Result<(), Failures> {
        match fork() {
            Ok(ForkResult::Parent { child, .. }) => {
                let result = waitpid(child, None);
                println!("{:?}", result);
            }
            Ok(ForkResult::Child) => {
                if cmd.background {
                    setpgid(Pid::from_raw(0), Pid::from_raw(0))?;
                }
                execvp(&cmd.bin(), &cmd.cargs())?;
            }
            Err(_) => println!("Fork failed"),
        }
        Ok(())
    }
}
