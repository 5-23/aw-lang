use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]
pub enum Token{
    Error(String),

    // type
    Ident(String),
    Str(String),
    Int(isize),
    Bool(bool),
}

impl Token {
    pub fn parse(ident: &str, var: &Mutex<HashMap<String, Self>>) -> Self {
        let ident = &ident.replace("%20", " ").trim().to_string();
        let chars = ident.chars().collect::<Vec<char>>();
        if ident == "true" || ident == "false" {
            return Self::Bool(ident.parse().unwrap())
        }
        match chars[0]{
            '0'..='9' | '-' => {
                let n = ident.parse::<isize>();
                if let Ok(n) = n{
                    Self::Int(n)
                }else{
                    Self::Error(format!("{:?} is not int!!", ident))
                }
            }
            '"' => {
                let c = chars[chars.len() - 1];
                if c != '"'{
                    Self::Error("String Not Closed!!".to_string())
                }else{
                    // let a = chars[1..=(chars.len()-1)];
                    let mut ident = ident.to_string();
                    ident.remove(0);
                    ident.remove(chars.len() - 2);
                    Self::Str(ident)
                }
            }
            _ => {
                let var = var.lock().unwrap();
                if var.contains_key(ident){
                    Self::Ident(ident.to_string())
                }else{
                    Self::Error("Variable Not Found!!".to_string())
                }
            }
        }
    }
}

impl std::fmt::Display for Token{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::Ident(v) = self{
            write!(f, "{}", v)
        }else if let Self::Str(v) = self{
            write!(f, "{}", v)
        }else if let Self::Int(v) = self{
            write!(f, "{}", v)
        }else if let Self::Bool(v) = self{
            write!(f, "{}", v)
        }else{
            write!(f, "")
        }
    }
}