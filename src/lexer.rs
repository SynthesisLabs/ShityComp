use std::ffi::c_void;
use std::str::FromStr;
use logos::{Logos};
#[derive(Logos, Debug, PartialEq)]
//This is the tokenizer its goal is to take a stream of code into a stream of tokens we can use 
pub enum Token {
    
    Number(i64), // Number token just is a 64bit int
    Plus,   //Plus operator +
    Minus,  //Minus operator -
    Mul,    //Multiplier operator *
    Div,    //Division operator /
    Mod,    //Modules operator %
    EOF,    //End of input
    Whitespace,
    Unknown,
}
pub struct Lexer{
    input: Vec<char>,
    position: usize,
}
impl Lexer{
    pub fn new(input: &str) -> Self{
        Lexer{
            input: input.chars().collect(),
            position: 0,
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
    pub fn what_is_next_char(&self)->Option<char>{
        self.input.get(self.position+1).copied()
    }

    pub fn next_token(&mut self)-> Token{
        
        while let Some(c) = self.current_char(){
            if(!c.is_whitespace()){
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
            '+' =>{
                self.advance();
                Token::Plus
            } ,
            '-' =>{
                self.advance();
                Token::Minus
            } ,
            '*' =>{
                self.advance();
                Token::Mul
            } ,
            '/' =>{
                self.advance();
                Token::Div
            } ,
            '%' =>{
                self.advance();
                Token::Mod
            } ,
                
            _ if char.is_ascii_digit() => {
                let mut num = String::new();
                while let Some(d) = self.current_char(){
                    if !d.is_ascii_digit(){
                        break;
                    }
                    num.push(d);
                    self.advance();
                }
                if let Ok(num) = num.parse::<i64>(){
                    return Token::Number(num)
                }else{
                    panic!("Invalid number given at: {} position number:{}",self.position, num);
                }

            }
            _ => panic!("Unexpected character: {} ", char),
        };
        return Token::Unknown
    }
}