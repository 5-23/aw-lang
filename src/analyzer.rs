use std::{collections::HashMap, sync::Mutex};
use lazy_static::lazy_static;

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
            input,
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
            if code == "=" { var_name = last_code }
            if last_code == "=" {
                VAR.lock().unwrap().insert(var_name.to_string(), Token::parse(code));
            }
            last_code = code
        }
        println!("{:?}", VAR.lock().unwrap())
    }
}
