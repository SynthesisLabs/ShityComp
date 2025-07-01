
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NodeType {
    NumericLiteral,
    String,
    Program,
}

struct NumericLiteral{
    node_type: NodeType,
    value: i64,
}
struct Program{
    node_type: NodeType,
    value : NumericLiteral,
}

pub struct Parser{
    input: String,
}
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
impl Parser{
    pub fn new(input: &str) -> Parser{
        Parser{
            input: input.to_string(),
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