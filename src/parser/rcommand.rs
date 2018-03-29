use std::ffi::CString;

pub struct RCommand {
    args: Vec<CString>,
    pub background: bool,
}

impl RCommand {
    pub fn new(buffer: &String) -> RCommand {
        let mut cmd_vec: Vec<CString> = buffer
            .split_whitespace()
            .map(|s| CString::new(s).unwrap())
            .collect();
        let background = match cmd_vec.last() {
            Some(elem) => {
                let el = elem.clone().into_string().unwrap();
                el == "&"
            }
            None => false,
        };

        if background {
            cmd_vec.pop();
        }
        RCommand {
            args: cmd_vec,
            background: background,
        }
    }

    pub fn bin(&self) -> CString {
        self.args[0].clone()
    }

    pub fn cargs(&self) -> &Vec<CString> {
        &self.args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_parse_test() {
        let rcommand_with_background = RCommand::new(&"ls -lah &".to_string());
        let rcommand_without_background = RCommand::new(&"ls -lah".to_string());
        assert_eq!(rcommand_with_background.background, true);
        assert_eq!(rcommand_without_background.background, false);
    }

    #[test]
    fn parse_test() {
        let rcommand = RCommand::new(&"ls -lah &".to_string());
        let cmd_args: Vec<String> = rcommand
            .cargs()
            .iter()
            .map(|s| s.clone().into_string().unwrap())
            .collect();
        assert_eq!(cmd_args, vec!["ls".to_string(), "-lah".to_string()]);
    }
}
