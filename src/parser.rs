use std::fmt;
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
struct Program{
    node_type: AstNodeType,
    value : AstNodeType,
}
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
    fn getCurrentToken(&self) -> Option<&Token>{
        self.tokens.get(self.pos)
    }
    fn advancePos(&mut self)-> Option<Token>{
        if self.pos < self.tokens.len(){
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(tok)
        }else{
            Some(Token::EOF)
        }
    }
    pub fn parse(&mut self){

        self.program();
        println!("Node type: {:?}",self.program().node_type);
        println!("Value: {} \n", self.program().value);
        println!("Numerical literal value: {}", self.program().value);
    }
    fn program(&mut self) -> Program{
        Program{
            node_type: AstNodeType::Program,
            value: self.numeric_literal(),
        }
    }
    fn numeric_literal(&mut self)-> AstNodeType{
        match self.advancePos(){
            Some(Token::Number(n)) =>{
                println!("Numerical literal value: {}", n);
                AstNodeType::NumericLiteral{value: n}
            }, 
            _=> panic!("Expected a num did not recieve")
        }
    }
}