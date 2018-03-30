use std::ffi::CString;

#[derive(Debug, Eq, PartialEq)]
pub struct RCommand {
    args: Vec<String>,
    pub background: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidCommand,
}

use self::Error::*;

impl RCommand {
    pub fn new(buffer: &String) -> Result<RCommand, Error> {
        if buffer.is_empty() {
            return Err(InvalidCommand);
        }
        let mut cmd_vec: Vec<String> = buffer.split_whitespace().map(|s| String::from(s)).collect();
        let background = match cmd_vec.last() {
            Some(elem) => elem == "&",
            None => false,
        };

        if background {
            cmd_vec.pop();
        }
        Ok(RCommand {
            args: cmd_vec,
            background: background,
        })
    }

    pub fn bin(&self) -> CString {
        CString::new(self.args[0].clone()).unwrap()
    }

    pub fn cargs(&self) -> Vec<CString> {
        self.args
            .iter()
            .map(|s| CString::new(&**s).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_parse_test() {
        let rcommand_with_background = RCommand::new(&"ls -lah &".to_string()).unwrap();
        let rcommand_without_background = RCommand::new(&"ls -lah".to_string()).unwrap();
        assert_eq!(rcommand_with_background.background, true);
        assert_eq!(rcommand_without_background.background, false);
    }

    #[test]
    fn parse_test() {
        let rcommand = RCommand::new(&"ls -lah &".to_string()).unwrap();
        assert_eq!(rcommand.args, vec!["ls", "-lah"]);
    }

    #[test]
    fn empty_cmd_test() {
        let rcommand = RCommand::new(&"".to_string());
        assert_eq!(rcommand, Err(InvalidCommand));
    }
}
