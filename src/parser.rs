use std::fmt;
use test::test_lexer;
use crate::lexer::Token;
use crate::test;

//derive traits for easy comparison and logging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//define the NodeType enum to keep NodeTypes organized
enum NodeType {
    NumericLiteral,
    String,
    Program,
}
//define NumericLiteral node
#[derive(Debug)]
struct NumericLiteral{
    node_type: NodeType,
    value: i64,
}
//define the Program node
struct Program{
    node_type: NodeType,
    value : NumericLiteral,
}
//define the parser its self
pub struct Parser{
    tokens : Vec<Token>,
    pos: usize,
}
//implement the display trait for the NodeType enum for easy logging
impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NodeType::NumericLiteral => "NumericLiteral",
            NodeType::Program => "Program",
            _ => {
                return Err(fmt::Error);
            }
        };
        write!(f, "{}", name)
    }
}
//implement the display trait for the NumericLiteral struct for easy logging
impl fmt::Display for NumericLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            NumericLiteral{node_type, value: _} => node_type,
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
    pub fn new() -> Parser{
        Parser{
            tokens: Vec::new(),
            pos: 0,
        }
    }
    pub fn parse(&mut self){

        self.program();
        println!("Node type: {:?}",self.program().node_type);
        println!("Value: {} \n", self.program().value);
        println!("Numerical literal value: {}", self.program().value.value);
    }
    fn program(&mut self) -> Program{
        Program{
            node_type: NodeType::Program,
            value: self.numeric_literal(),
        }
    }
    fn numeric_literal(&mut self)-> NumericLiteral{
        NumericLiteral{
            node_type: NodeType::NumericLiteral,
            value: self.input.trim().parse::<i64>().unwrap(),
        }
    }
}