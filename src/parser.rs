

struct NumericLiteral{
    node_type: String,
    value: i64,
}

pub struct Parser{
    input: String,
}
impl Parser{
    pub fn new(input: &str) -> Parser{
        Parser{
            input: input.to_string(),
        }
    }
    pub fn parse(&mut self){
        self.program()
    }
    pub fn program(&mut self){
        self.numeric_literal();
        println!("Number of literals: {}", self.input.len());
        println!("Node type: {}",self.numeric_literal().node_type);
        println!("Value: {}", self.numeric_literal().value) 
    }
    pub fn numeric_literal(&mut self)-> NumericLiteral{
        NumericLiteral{
            node_type: "NumericLiteral".to_string(),
            value: self.input.trim().parse::<i64>().unwrap(),
        }
    }
}