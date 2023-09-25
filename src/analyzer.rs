use std::{collections::HashMap, sync::Mutex, io::Write};
use regex::Regex;
use lazy_static::lazy_static;
use colorfully::*;

use crate::token::Token;

lazy_static! {
    static ref VAR: Mutex<HashMap<String, Token>> = {
        Mutex::new(HashMap::new())
    };
}


pub struct Analyzer<'a> {
    input: &'a str,
    ch: u8
}
impl<'a> Analyzer<'a> {
    pub fn new(input: &'a str) {
        let an = Analyzer {
            input: &input.replace("\\ ", "%20"),
            ch: 0,
        };
        an.run_analyze();
    }
    fn run_analyze(&self) {
        let codes = self.input.trim().replace("\n", " ");
        let codes = codes.split(" ");
        let mut line = 0;
        let mut last_code = "";
        let mut var_name = "";
        for code in codes {
            line += 1;


            // TODO: make var
            if code.contains("var") {
                let re: Regex = Regex::new(r"var\((?<code>\d.*)\)").unwrap();
                println!("{:?}", &re.captures(code).unwrap()["code"]);
            }

            if code == "=" { var_name = last_code }
            if last_code == "=" {
                let token = Token::parse(code, &VAR);
                if let Token::Error(error) = &token{
                    Self::print_error(line, error);
                    return;
                }
                VAR.lock().unwrap().insert(var_name.to_string(), token);
            }

            if last_code == "print"{
                let token = Token::parse(code, &VAR);
                if let Token::Error(error) = &token {
                    Self::print_error(line, error);
                    return;
                }

                if let Token::Ident(name) = &token {
                    print!("{}", VAR.lock().unwrap().get(name).unwrap());
                }
                if let Token::Str(v) = &token {
                    print!("{}",v);
                }
                if let Token::Int(v) = &token {
                    print!("{}",v);
                }
                if let Token::Bool(v) = &token {
                    print!("{}",v);
                }
                std::io::stdout().flush().unwrap();
            }

            if last_code == "println"{
                let token = Token::parse(code, &VAR);
                if let Token::Error(error) = &token {
                    Self::print_error(line, error);
                    return;
                }

                if let Token::Ident(name) = &token {
                    println!("{}", VAR.lock().unwrap().get(name).unwrap());
                }
                if let Token::Str(v) = &token {
                    println!("{}",v);
                }
                if let Token::Int(v) = &token {
                    println!("{}",v);
                }
                if let Token::Bool(v) = &token {
                    println!("{}",v);
                }
            }

            last_code = code
        }
        // println!("{:?}", VAR.lock().unwrap())
    }

    fn print_error(line: usize, error: &String){
        print!("{}{}{}", "Error[".bold().red(), format!("{line}] ").bold().red(), error.gray())
    }
}
