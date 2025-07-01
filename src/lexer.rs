use std::str::FromStr;
use logos::{Logos};
#[derive(Logos, Debug, PartialEq)]
//This is the tokenizer its goal is to take a stream of code into a stream of tokens we can use 
pub enum Token {
    Float(f64),
    Number(i64), // Number token just is a 64bit int
    Plus,   //Plus operator +
    Minus,  //Minus operator -
    Mul,    //Multiplier operator *
    Div,    //Division operator /
    Mod,    //Modules operator %
    EOF,    //End of input
    Err,
    Whitespace,
    Unknown,
}

pub struct Lexer{
    input: Vec<char>,
    position: usize,
    pub tokens: Vec<Token>,
}
impl Lexer{
    pub fn new(input: &str) -> Self{
        Lexer{
            input: input.chars().collect(),
            position: 0,
            tokens: Vec::new(),
        }
    }
    fn current_char(&self)->Option<char>{
        self.input.get(self.position).copied()
    }
    
    //function to decide if the end of the input hase been reached
    fn eof(&self)->bool{
        if self.position >= self.input.len() {
            true
        }else{
            false
        }
    }
    fn advance(&mut self){
        self.position += 1;
    }
    pub fn what_is_next_char(&self)->Result<Option<char>,Token>{
        let next_pos = self.position+1;
        if self.input.len()  > next_pos{
            let next_pos = self.position +1;
            let next_char = self.input.get(next_pos).copied();
            Ok(next_char)
        }else{
            Err(Token::EOF)
        }
        
    }
    pub fn what_is_char_at(&self, offset: usize) -> Result<Option<char>,Token>{
        let new_pos = self.position+offset;
        if new_pos >= self.input.len() {
            let result = self.input.get(new_pos).copied();
            Ok(result)

        }else{
            println!("Position out of range");
            Err(Token::EOF)
        }
        
    }

    pub fn next_token(&mut self)-> Token{

        while let Some(c) = self.current_char(){
            if !c.is_whitespace() {
                break;
            }
            self.advance();
            return Token::Whitespace;
        }
        if self.eof(){
            return Token::EOF
        }

        let char = self.current_char().unwrap();
        match char {
            '+' => {
                self.advance();
                Token::Plus
            },
            '-' => {
                let mut is_num = true;
                let mut num = String::new();
                let mut index = 0;
                while is_num{
                    index += 1;
                    if index == 1{
                        num.push(char);
                    }
                    match self.what_is_next_char(){
                        Ok(Some(c)) if c.is_whitespace() => {
                            self.advance();
                            return Token::Minus;
                        }
                        Ok(Some(c)) if c.is_numeric() => {
                            num.push(c);
                            self.advance();
                            println!("Printed num {}", num);
                            is_num = true;
                        }
                        Ok(Some(_)) | Ok(None) => { is_num = false; }
                        Err(token) => {
                            is_num = false;
                            return if !num.is_empty() {
                                println!("Empty NOT num");
                                Token::Number(i64::from_str(&num).unwrap())
                            }else{
                                token
                            }
                        }
                    }
                }
                return Token::Err;
            },
            '*' => {
                self.advance();
                Token::Mul
            },
            '/' => {
                self.advance();
                Token::Div
            },
            '%' => {
                self.advance();
                Token::Mod
            },
            '.' =>{
                match self.what_is_next_char() {
                    Ok(Some(c)) if c.is_ascii_digit() => {
                        self.parse_nums()
                    }
                    Err(token) => return token,
                    _ => return Token::Err,
                }
            }
            char  if char.is_ascii_digit() => {
                let test = self.parse_nums();
                return test
            },
            _ => {

                eprint!("unexpected character {}", char);
                return Token::Unknown
            }
        }
    }
    fn parse_nums(&mut self)-> Token{
        let mut num = String::new();
        let mut contains_dot = false;
        
        //check if the current number contains a dot
        if let Some('.') = self.current_char() {
            contains_dot = true;
            num.push('.');
            self.advance();
        }
        //check for integers and a check for if the integers check fails
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                println!("Char is a num");
                num.push(c);
                self.advance();
            }else if c == '.' && !contains_dot {
                contains_dot = true;
                num.push('.');
                self.advance();
            }else{
                break;
            }
            //checks for scientific notations
            if let Some(c) = self.current_char() {
                if c  == 'e' || c == 'E' {
                    num.push(c);
                    self.advance();
                    if let Some(sign) = self.current_char() {
                        if sign == '+' || sign == '-' {
                            num.push(sign);
                            self.advance();
                        }
                    }
                }
                //check to see if after the scientific notation there are numbers if not return an error due illegal notation
                let mut hase_exponents = false;
                while let Some(c) = self.current_char() {
                    if c.is_ascii_digit(){
                        num.push(c);
                        self.advance();
                        hase_exponents = true;
                    }else{
                        break;
                    }
                    println!("Expo loop")
                }
                if !hase_exponents {
                    return Token::Err;
                }
            }
            println!("Hase dot: {}",contains_dot);
            // actually parse the string to an int/float and return them
            return if contains_dot || num.contains('e') || num.contains('E') {
                match num.parse::<f64>() {
                    Ok(f) => Token::Float(f),
                    Err(_) => Token::Err,
                }
            } else {
                match num.parse::<i64>() {
                    Ok(i) => Token::Number(i),
                    Err(_) => {
                        match num.parse::<f64>() {
                            Ok(float) => Token::Float(float),
                            Err(_) => Token::Err,
                        }
                    }
                }
            }

        }
        return Token::Err;
    }
}