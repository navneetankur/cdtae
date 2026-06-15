use crate::command::Command;

pub fn parse<'a>(input: &'a str) -> ParseResult<Command<'a>> {
    todo!()
}

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("malformed replace")]
    MalformedReplace,
}
type ParseResult<T> = Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use crate::command;
use super::*;
    #[test]
    fn t_parse_undo() {
        let output = parse("undo");
        assert_eq!(output, Ok(Command::Undo));
    }
    #[test]
    fn t_parse_redo() {
        let output = parse("redo");
        assert_eq!(output, Ok(Command::Redo));
    }
    #[test]
    fn t_parse_replace() {
        let output = parse("replace x with y");
        assert_eq!(output, Ok(Command::Replace(command::Replace{ 
            from: "x",
            to: "y",
        })));
    }
}
