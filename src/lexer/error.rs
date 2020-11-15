use std::fmt;

#[derive(Debug)]
pub enum LexicalError {
    InvalidStringLiteral(usize, usize),
    InvalidIntegerConstant(usize, usize, String),
    UnknownToken(usize, usize),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::UnknownToken(_, _) => write!(f, "[Error]: Unknown Token"),
            LexicalError::InvalidStringLiteral(_, _) => {
                write!(f, "[Error]: Invalid String Literal")
            }
            LexicalError::InvalidIntegerConstant(_, _, constant) => {
                write!(f, "[Error]: Aah number ivide irakanda {}", constant)
            }
        }
    }
}
