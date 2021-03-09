use nom::error::{ContextError, ErrorKind, ParseError};

#[derive(Debug)]
pub struct ParserError {
    context: &'static str,
    message: Option<String>,
}

impl ParserError {
    pub fn new(context: &'static str, message: String) -> Self {
        Self {
            context,
            message: Some(message),
        }
    }
}

impl<I> ParseError<I> for ParserError {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Self {
            context: "Nom Error",
            message: Some(kind.description().to_string()),
        }
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> ContextError<I> for ParserError {
    fn add_context(_: I, context: &'static str, other: Self) -> Self {
        match other.context {
            "Nom Error" => Self {
                context,
                message: None,
            },
            _ => other,
        }
    }
}

impl From<ParserError> for nom::Err<ParserError> {
    fn from(error: ParserError) -> Self {
        nom::Err::Failure(error)
    }
}
