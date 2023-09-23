#[derive(Debug)]
pub enum Token{
    // type
    Ident(String),
    Str(String),
    Int(isize),


    // operators
    Assign,
    Plus,
    Minus,
    Asterisk,
    Slash,

    Eq,
    NotEq,

    // Reserved
    Function,
    Let,
    If,
    Else,
    Return,
}

impl Token {
    pub fn parse(ident: &str) -> Self {
        match ident.chars().collect::<Vec<char>>()[0]{
            '0'..='9' => {
                Self::Int(ident.parse().unwrap())
            }
            _ => Self::Str(ident.to_string())
        }
    }
}