use crate::{command::Command, env::Env};

pub fn parse<'a>(env: &Env, input: &'a str) -> ParseResult<Command<'a>> {
    match replace(env, input) {
        MaybeCommand::Command(cmd) => return Ok(cmd),
        MaybeCommand::Error(e) => return Err(e),
        MaybeCommand::None => {}
    }
    match input.trim() {
        "undo" => Ok(Command::Undo),
        "redo" => Ok(Command::Redo),
        "write" => Ok(Command::Write),
        _ => todo!(),
    }
}

fn replace<'a>(env: &Env, input: &'a str) -> MaybeCommand<'a> {
    let words: Vec<&str> = input.split_whitespace().collect();
    if words.len() < 2 {
        return MaybeCommand::None;
    }
    let replace_words = &env.words.replace;
    let verb_matches = if replace_words.is_empty() {
        words[0] == "replace"
    } else {
        replace_words.iter().any(|w| w == words[0])
    };
    if !verb_matches {
        return MaybeCommand::None;
    }
    if words.len() != 4 || !env.words.with.iter().any(|w| w == words[2]) {
        return MaybeCommand::Error(ParseError::MalformedReplace);
    }
    MaybeCommand::Command(Command::Replace(crate::command::Replace {
        from: words[1],
        to: words[3],
    }))
}


enum MaybeCommand<'a> {
    Command(Command<'a>),
    None,
    Error(ParseError),
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("malformed replace")]
    MalformedReplace,
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
