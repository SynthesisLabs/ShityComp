use std::fmt;
use std::ptr::null;
use test::test_lexer;
use crate::lexer::Token;
use crate::test;

//derive traits for easy comparison and logging
#[derive(Debug, Clone, PartialEq)]
//define the NodeType enum to keep NodeTypes organized
pub enum AstNodeType {
    NumericLiteral{value: i64},
    String(String),
    BinaryExpression {
        left: Box<AstNodeType>,
        operator: BinaryOperator,
        right: Box<AstNodeType>,
    },
    Program{
        body: Vec<AstNodeType>,
    },
}
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}
//define the Program node

//define the parser its self
pub struct Parser{
    tokens : Vec<Token>,
    pos: usize,
}

//implement the actual parser
impl Parser{
    //create a constructor to set input on object creation
    pub fn new(input: String) -> Parser{
        
        Parser{
            tokens: test_lexer(&*input).expect("Lexer failed"),
            pos: 0,
        }
    }
    fn get_current_token(&self) -> Option<&Token>{
        self.tokens.get(self.pos)
    }
    fn advance_pos(&mut self) -> Token{
        if self.pos < self.tokens.len(){
            let tok = self.tokens[self.pos].clone();
            println!("advance_pos() -> {:?}", tok);
            self.pos += 1;
            tok
        }else{
            Token::EOF
        }
    }
    fn skip_whitespace(&mut self) {
        while matches!(self.get_current_token(), Some(Token::Whitespace)) {
            self.advance_pos();
        }
    }
    pub fn parse(&mut self) -> AstNodeType{
        for i in self.tokens.iter(){
            println!("parse() -> {:?}", i);

        }

        return self.program()

    }
    fn program(&mut self) -> AstNodeType{
        let mut body = Vec::new();
        loop {
            self.skip_whitespace();
            match self.get_current_token() {
                Some(Token::EOF) | None => {
                    return AstNodeType::Program { body };
                }
                _ => body.push(self.parse_expression()),
            }
        }
    }

    fn parse_expression(&mut self) -> AstNodeType {
        self.parse_additive()

    }

    fn parse_additive(&mut self) -> AstNodeType {
        let mut left = self.parse_multiplicative();

        loop {
            self.skip_whitespace();
            let op = match self.get_current_token() {
                Some(Token::Plus) => BinaryOperator::Add,
                Some(Token::Minus) => BinaryOperator::Subtract,
                _ => break,
            };
            self.advance_pos();
            let right = self.parse_multiplicative();
            left = AstNodeType::BinaryExpression {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_multiplicative(&mut self) -> AstNodeType {
        let mut left = self.parse_primary();

        loop {
            self.skip_whitespace();
            let op = match self.get_current_token() {
                Some(Token::Mul) => BinaryOperator::Multiply,
                Some(Token::Div) => BinaryOperator::Divide,
                Some(Token::Mod) => BinaryOperator::Modulo,
                _ => break,
            };
            self.advance_pos();
            let right = self.parse_primary();
            left = AstNodeType::BinaryExpression {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_primary(&mut self) -> AstNodeType {
        self.skip_whitespace();
        match self.get_current_token() {
            Some(Token::Number(_)) => self.parse_numeric_literal(),
            Some(Token::String(_)) => self.parse_string_literal(),
            Some(Token::Err) => {
                let tok = self.advance_pos();
                panic!("lexer error token: {:?}", tok);
            }
            Some(Token::EOF) | None => panic!("unexpected EOF while parsing expression"),
            _ => {
                let tok = self.advance_pos();
                panic!("unexpected token: {:?}", tok)
            }
        }
    }

    fn parse_numeric_literal(&mut self) -> AstNodeType {
         match self.advance_pos() {
            Token::Number(n) => AstNodeType::NumericLiteral{value: n},
            other => panic!("expected number, found {:?}", other),
        }
    }

    fn parse_string_literal(&mut self) -> AstNodeType {
        match self.advance_pos() {
            Token::String(s) => AstNodeType::String(s),
            other => panic!("expected string, found {:?}", other),
        }
    }
}