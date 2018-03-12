// pub enum Constant {
//     String(String),
// }

// pub enum Token {
//     KwIf,
//     KwThen,
//     KwElse,
//     Constant(Constant),
// }


// impl Token {
//     pub fn literal(&self) ->  Option<&'static str> {
//         match self {
//             &Token::KwIf => Some("if"),
//             &Token::KwThen => Some("then"),
//             &Token::KwElse => Some("else"),
//             _ => None,
//         }
//     }
// }

// use Constant::*;
// pub fn print_token(token: Token) {
//     match token.literal() {
//         Some(literal) => print!("{}", literal),
//         None => match token {
//             Token::Constant(constant) => match constant {
//                 String(string) => print!("{}", string),
//             }
//         }
//     }
// }

fn main() {
    let f = "Hello";
    // let x = concat!(f, f);
    // let y = format_args!(f);
    std::io::_print(f);
    // print!(f);
}

