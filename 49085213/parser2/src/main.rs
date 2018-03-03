pub enum Constant {
    String(String),
}

pub enum Token {
    KwIf,
    KwThen,
    KwElse,
    Constant(Constant),
}


impl Token {
    pub fn literal(&self) ->  Option<&'static str> {
        match self {
            &Token::KwIf => Some("if"),
            &Token::KwThen => Some("then"),
            &Token::KwElse => Some("else"),
            _ => None,
        }
    }
}

use Constant::*;
pub fn print_token(token: Token) {
    match token.literal() {
        Some(literal) => print!("{}", literal),
        None => match token {
            Token::Constant(constant) => match constant {
                String(string) => print!("{}", string),
            }
        }
    }
}

fn main() {
    let t = Token::Constant::String("Hello");
}
