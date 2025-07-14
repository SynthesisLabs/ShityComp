use std::fmt;
use std::ptr::null;
use test::test_lexer;
use crate::lexer::Token;
use crate::test;

//derive traits for easy comparison and logging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//define the NodeType enum to keep NodeTypes organized
enum AstNodeType {
    NumericLiteral{value: i64},
    String,
    Program,
}
//define the Program node

//define the parser its self
pub struct Parser{
    tokens : Vec<Token>,
    pos: usize,
}
//implement the display trait for the NodeType enum for easy logging
impl fmt::Display for AstNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            AstNodeType::Program => "Program",
            _ => {
                return Err(fmt::Error);
            }
        };
        write!(f, "{}", name)
    }
}
//implement the actual parser
impl Parser{
    //create a constructor to set input on object creation
    pub fn new(input: String) -> Parser{
        
        Parser{
            tokens: test_lexer(&*input).expect("Oke"),
            pos: 0,
        }
    }
    fn get_current_token(&self) -> Option<&Token>{
        self.tokens.get(self.pos)
    }
    fn advancePos(&mut self)-> Option<Token>{
        if self.pos < self.tokens.len(){
            let tok = self.tokens[self.pos].clone();
            println!("advancePos() -> {:?}", tok);
            self.pos += 1;
            Some(tok)
        }else{
            Some(Token::EOF)
        }
    }
    pub fn parse(&mut self){
        for i in self.tokens.iter(){
            println!("parse() -> {:?}", i);
        }
        let node = self.program();
        match node{
            AstNodeType::NumericLiteral { value } => {
                println!("Value: {}", value);
            }
            _ =>{
                println!("EOF");
            }
        }
    }
    fn program(&mut self) -> AstNodeType{
        let node = self.numeric_literal();
        match self.advancePos(){
            Some(Token::EOF)=>node,
            Some(Token::Whitespace)=>node,
            Some(tok)=> panic!("Unexpected token {:?}", tok),
            None => node,
        }
    }
    fn numeric_literal(&mut self)-> AstNodeType{
        if let Some(token) = self.advancePos(){
            loop{
                match token{
                    Token::Number(n) =>{
                        println!("Numerical literal {}", n);
                        return AstNodeType::NumericLiteral{value: n};
                    },
                    Token::Whitespace =>{
                        continue;
                    }
                    Token::Err =>{
                        continue;
                    }
                    Token::EOF =>{
                        continue;
                    }
                    _ =>{
                    println!("token {:?}", token);
                    panic!("Expected a num did not recieve at pos {}", self.pos);
                    }
                }
            }

        }
        panic!("Unexpected EOF");

    }
}