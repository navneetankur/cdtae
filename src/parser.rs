use suffixes::wrap::WrappedErrResult;

use crate::{command::{Command, Replace}, env::Env, ext::ContainsStr};

pub fn parse<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    match replace(env, input) {
        Ok(c) => return Ok(c),
        Err(e) if e == ParseError::NotMe => {/*continue trying other commands*/},
        Err(e) => return e.err(),
    }
    match input.trim() {
        "undo" => Ok(Command::Undo),
        "redo" => Ok(Command::Redo),
        "write" => ParseError::Todo("write".into()).err(),
        _ => unimplemented!(),
    }
}

fn replace<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    let mut input = input.split_whitespace();
    let replace_p = input.next().ok_or(ParseError::NotMe)?;
    let words = &env.words;
    if !words.replace.contains_str(replace_p) {return ParseError::NotMe.err()};
    let from = input.next().ok_or(ParseError::Malformed)?;
    let with_p = input.next().ok_or(ParseError::Malformed)?;
    if !words.with.contains_str(with_p) {return ParseError::Malformed.err()};
    let to = input.next().ok_or(ParseError::Malformed)?;
    return Ok(Command::Replace(Replace {
        from,
        to,
    }));
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("malformed")]
    Malformed,
    #[error("not me")]
    NotMe, //think better name. it should say command type is not the one you are trying.
    #[error("todo: {0}")]
    Todo(String),
}
type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use crate::{command, env::Env};
use super::*;
    #[test]
    fn t_parse_undo() {
        let env = Env::default();
        let output = parse(&env, "undo");
        assert_eq!(output, Ok(Command::Undo));
    }
    #[test]
    fn t_parse_redo() {
        let output = parse(&Env::default(), "redo");
        assert_eq!(output, Ok(Command::Redo));
    }
    #[test]
    fn t_parse_replace() {
        let output = parse(&Env::default(), "replace x with y");
        assert_eq!(output, Ok(Command::Replace(command::Replace{ 
            from: "x",
            to: "y",
        })));
    }
}
