use std::ffi::c_void;
use logos::{Logos};
#[derive(Logos, Debug, PartialEq)]
//This is the tokenizer its goal is to take a stream of code into a stream of tokens we can use 
pub enum Token {
    #[regex(r"\d+")] Number,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Mul,
    #[token("/")] Div,
    #[token("%")] Mod,
}
fn test() {
}