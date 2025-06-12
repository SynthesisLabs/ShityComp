use std::ffi::c_void;
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
    pub fn next_token(&mut self)-> Token{
        while let Some(c) = self.current_char(){
            if(!c.is_whitespace()){
                break;
            }
            self.advance();
        }
        if self.eof(){
            return Token::EOF
        }
        return Token::Unknown
    }
}