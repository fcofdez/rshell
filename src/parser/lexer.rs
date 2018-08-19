use std::borrow::Cow;
use std::char;
use std::str;
use std::string;

#[derive(Eq, PartialEq, Debug)]
pub enum Error {
    Unexpected(usize, char),
}

