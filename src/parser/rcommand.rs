use std::ffi::CString;

pub struct RCommand {
    pub bin: CString,
    pub args: Vec<CString>,
    background: bool,
}

impl RCommand {
    pub fn new(buffer: &String) -> RCommand {
        let cmd = buffer.replace("\n", "");
        let cmd_vec: Vec<CString> = cmd.split(" ").map(|s| CString::new(s).unwrap()).collect();
        let args: Vec<CString> = cmd_vec[1..cmd_vec.len()].to_vec();
        RCommand {
            bin: cmd_vec[0].clone(),
            args: args,
            background: false,
        }
    }
}
