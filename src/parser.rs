
struct NumericLiteral{
    node_type: String,
    value: i64,
}

pub struct Parser{
    input: Vec<char>,
}
impl Parser{
    pub fn new(input: &str) -> Parser{
        Parser{
            input: input.chars().collect(),
        }
    }
    
}