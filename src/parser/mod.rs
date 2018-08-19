#[macro_use]
pub mod rcommand;

use nom::{digit, is_alphabetic, IResult};

use nom::types::CompleteByteSlice;
use nom::types::CompleteStr;
use std::str;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Token {
    GreaterThan,
    LessThan,
    DoubleLessThan,
    DoubleGreaterThan,
    LessThanAmp,
    GreaterThanAmp,
    Minus,
    LessLessMinus,
    LessGreater,
    GreaterPipe,
    Pipe,
    DoublePipe,
    LParen,
    RParen,
    LBrace,
    RBrace,
    SemiColon,
    DoubleSemiColon,
    LBracket,
    RBracket,
    Ampersand,
    DoubleAmpersand,
    ExclamationMark,
    If,
    Else,
    ElIf,
    EndIf,
    For,
    Do,
    Done,
    Case,
    Esac,
    While,
    Until,
    Function,
    Select,
    Word(String),
    Number(String),
    Illegal,
    EOF
}

fn is_alpha_or_underscore(c: u8) -> bool {
    is_alphabetic(c) || c == b'_'
}
//
//fn complete_byte_slice_to_str(s: CompleteByteSlice) -> Result<&str, str::Utf8Error> {
//    str::from_utf8(s.0)
//}

// Taken from -> https://github.com/Rydgel/monkey-rust/blob/master/lib/lexer/mod.rs
macro_rules! check(
  ($input:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use std::result::Result::*;
      use nom::{Err,ErrorKind};

      let mut failed = false;
      for &idx in $input.0 {
        if !$submac!(idx, $($args)*) {
            failed = true;
            break;
        }
      }
      if failed {
        let e: ErrorKind<u32> = ErrorKind::Tag;
        Err(Err::Error(error_position!($input, e)))
      } else {
        Ok((&b""[..], $input))
      }
    }
  );
  ($input:expr, $f:expr) => (
    check!($input, call!($f));
  );
);

fn complete_byte_slice_str_from_utf8(c: CompleteByteSlice) -> Result<CompleteStr, str::Utf8Error> {
    str::from_utf8(c.0).map(|s| CompleteStr(s))
}

named!(word<CompleteByteSlice, Token>,
    do_parse!(
        letter: map_res!(
              flat_map!(take!(1), check!(is_alphabetic)),
                        complete_byte_slice_str_from_utf8) >>
        rest: opt!(
               complete!(
                  map_res!(take_while!(is_alpha_or_underscore),
                         complete_byte_slice_str_from_utf8))) >>
        (parse_reserved(letter, rest))
    )
);

fn parse_reserved(letter: CompleteStr, rest: Option<CompleteStr>) -> Token {
    let mut string = letter.0.to_owned();
    string.push_str(rest.unwrap_or(CompleteStr("")).0);

    match string.as_ref() {
        "if" => Token::If,
        "else" => Token::Else,
        "elif" => Token::ElIf,
        "fi" => Token::EndIf,
        "for" => Token::For,
        "do" => Token::Do,
        "done" => Token::Done,
        "case" => Token::Case,
        "esac" => Token::Esac,
        "while" => Token::While,
        "until" => Token::Until,
        "function" => Token::Function,
        "select" => Token::Select,
        _ => Token::Word(string),
    }
}

named!(number<CompleteByteSlice, Token>,
  do_parse!(
    n: map_res!(digit, complete_byte_slice_str_from_utf8) >>
    (Token::Number(n.to_string()))
  )
);

// Redirection operators
named!(greater_operator<CompleteByteSlice, Token>,
  do_parse!(tag!(">") >> (Token::GreaterThan))
);

named!(less_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("<") >> (Token::LessThan))
);

named!(double_less_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("<<") >> (Token::DoubleLessThan))
);

named!(double_greater_operator<CompleteByteSlice, Token>,
  do_parse!(tag!(">>") >> (Token::DoubleGreaterThan))
);

named!(less_amp_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("<&") >> (Token::LessThanAmp))
);

named!(greater_amp_operator<CompleteByteSlice, Token>,
  do_parse!(tag!(">&") >> (Token::GreaterThanAmp))
);

named!(minus_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("-") >> (Token::Minus))
);

named!(less_less_minus_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("<<-") >> (Token::LessLessMinus))
);

named!(less_greater_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("<>") >> (Token::LessGreater))
);

named!(greater_pipe_operator<CompleteByteSlice, Token>,
  do_parse!(tag!(">|") >> (Token::GreaterPipe))
);

named!(redirect_operator<CompleteByteSlice, Token>, alt!(
    greater_operator |
    less_operator |
    double_less_operator |
    double_greater_operator |
    less_amp_operator |
    greater_amp_operator |
    minus_operator |
    less_less_minus_operator |
    less_greater_operator |
    greater_pipe_operator
));

// Other operators

named!(pipe_operator<CompleteByteSlice, Token>,
  do_parse!(tag!("|") >> (Token::Pipe))
);

named!(semicolon<CompleteByteSlice, Token>,
  do_parse!(tag!(";") >> (Token::SemiColon))
);

named!(double_semicolon<CompleteByteSlice, Token>,
  do_parse!(tag!(";;") >> (Token::DoubleSemiColon))
);

named!(lparen<CompleteByteSlice, Token>,
  do_parse!(tag!("(") >> (Token::LParen))
);

named!(rparen<CompleteByteSlice, Token>,
  do_parse!(tag!(")") >> (Token::RParen))
);

named!(lbrace<CompleteByteSlice, Token>,
  do_parse!(tag!("{") >> (Token::LBrace))
);

named!(rbrace<CompleteByteSlice, Token>,
  do_parse!(tag!("}") >> (Token::RBrace))
);

named!(lbracket<CompleteByteSlice, Token>,
  do_parse!(tag!("[") >> (Token::LBracket))
);

named!(rbracket<CompleteByteSlice, Token>,
  do_parse!(tag!("]") >> (Token::RBracket))
);

named!(ampersand<CompleteByteSlice, Token>,
  do_parse!(tag!("&") >> (Token::Ampersand))
);

named!(double_ampersand<CompleteByteSlice, Token>,
  do_parse!(tag!("&&") >> (Token::DoubleAmpersand))
);

named!(double_pipe<CompleteByteSlice, Token>,
  do_parse!(tag!("||") >> (Token::DoublePipe))
);

named!(exclamation_mark<CompleteByteSlice, Token>,
  do_parse!(tag!("!") >> (Token::ExclamationMark))
);

named!(operator<CompleteByteSlice, Token>, alt!(
    pipe_operator |
    semicolon |
    double_semicolon |
    lparen |
    rparen |
    lbrace |
    rbrace |
    lbracket |
    rbracket |
    ampersand |
    double_ampersand |
    double_pipe |
    exclamation_mark
));

// Illegal tokens
named!(illegal<CompleteByteSlice, Token>,
    do_parse!(take!(1) >> (Token::Illegal))
);

named!(token<CompleteByteSlice, Token>, alt_complete!(
    operator |
    redirect_operator |
    word |
    number |
    illegal
));

named!(lex_tokens<CompleteByteSlice, Vec<Token>>, ws!(many0!(token)));

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<CompleteByteSlice, Vec<Token>> {
        lex_tokens(CompleteByteSlice(bytes)).map(|(slice, result)|
            (slice, [&result[..], &vec![Token::EOF][..]].concat())
        )
    }
}

#[test]
fn test_word() {
    let (_, result) = Lexer::lex_tokens(&b"ab_cd ab_ abcd a"[..])
        .unwrap();

    let expected = vec![
        Token::Word("ab_cd".to_owned()),
        Token::Word("ab_".to_owned()),
        Token::Word("abcd".to_owned()),
        Token::Word("a".to_owned()),
        Token::EOF,
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_number() {
    let (_, result) = Lexer::lex_tokens(&b"123 0"[..])
        .unwrap();

    let expected = vec![
        Token::Number("123".to_owned()),
        Token::Number("0".to_owned()),
        Token::EOF,
    ];

    assert_eq!(result, expected);
}

#[test]
fn test_simple_tokens() {
    let (_, result) = Lexer::lex_tokens(&b"> <"[..])
        .unwrap();

    let expected = vec![
        Token::GreaterThan,
        Token::LessThan,
        Token::EOF,
    ];

    assert_eq!(result, expected);
}
