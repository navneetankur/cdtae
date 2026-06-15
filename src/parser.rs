use crate::command::Command;
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("malformed replace")]
    MalformedReplace,
}
type ParseResult<T> = Result<T, ParseError>;

pub fn parse(input: &str) -> ParseResult<Command> {
    todo!()
}
#[cfg(test)]
mod tests {
    use crate::command;

use super::*;

    #[test]
    fn t_parse_undo() {
        let output = parse("undo");
        assert_eq!(output, Ok(Command::Undo));
    }
}
