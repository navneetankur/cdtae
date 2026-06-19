use suffixes::{WrappedResult, wrap::WrappedErrResult};

use crate::{
    command::{Command, Replace},
    env::Env,
    ext::ContainsStr,
};

pub fn parse<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    match replace(env, input) {
        Err(ParseError::NotMe) => { /*continue trying other commands*/ }
        Err(e) => return e.err(),
        Ok(c) => return Ok(c),
    }
    match undo(env, input) {
        Err(ParseError::NotMe) => { /*continue trying other commands*/ }
        Err(e) => return e.err(),
        Ok(c) => return Ok(c),
    }
    match redo(env, input) {
        Err(ParseError::NotMe) => { /*continue trying other commands*/ }
        Err(e) => return e.err(),
        Ok(c) => return Ok(c),
    }
    return Ok(Command::Write(input));
}

fn redo<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    if env.words.redo.contains_str(input) {
        Command::Redo.ok()
    } else {
        ParseError::NotMe.err()
    }
}
fn undo<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    if env.words.undo.contains_str(input) {
        Command::Undo.ok()
    } else {
        ParseError::NotMe.err()
    }
}
fn replace<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    let mut input = input.split_whitespace();
    let replace_p = input.next().ok_or(ParseError::NotMe)?;
    let words = &env.words;
    if !words.replace.contains_str(replace_p) {
        return ParseError::NotMe.err();
    }
    let from = input.next().ok_or(ParseError::Malformed)?;
    let with_p = input.next().ok_or(ParseError::Malformed)?;
    if !words.with.contains_str(with_p) {
        return ParseError::Malformed.err();
    }
    let to = input.next().ok_or(ParseError::Malformed)?;
    return Ok(Command::Replace(Replace { from, to }));
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
    use super::*;
    use crate::{command, env::Env};
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
        assert_eq!(
            output,
            Ok(Command::Replace(command::Replace { from: "x", to: "y" }))
        );
    }
    #[test]
    fn t_parse_replace_err() {
        let output = parse(&Env::default(), "replace x y");
        assert_eq!(output, Err(ParseError::Malformed));
    }
    #[test]
    fn t_parse_replace_notme() {
        let output = replace(&Env::default(), "Twinkle twinkle");
        assert_eq!(output, Err(ParseError::NotMe));
    }
}
