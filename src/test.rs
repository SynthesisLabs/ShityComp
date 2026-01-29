use crate::lexer;
use crate::lexer::Token;
use crate::parser::Parser;

//file containing all test methods
pub fn test_lexer(input: &str)-> Result<Vec<Token>, String> {
    //initialize lexer
    let mut lexer = lexer::Lexer::new(input);
    loop{
        let token = lexer.next_token();
        println!("{:?} \n", token);
        match token{
            Token::EOF =>{
                if(lexer.tokens.is_empty()){
                    return Err("No tokens were parsed empty input".to_string());
                }
                println!("End of file reached");
                println!("Tokenlist: {:?} \n", lexer.tokens);
                return Ok(lexer.tokens);
            }
            Token::Err=>{
                return Err("Unexpected Error".to_string());
            }
            _=>{
                lexer.tokens.push(token);
            }
        }

    }

}
pub fn test_parser(input: &str){
    
    let mut parser = Parser::new(input.parse().unwrap());
    println!(" ");
    println!("Parser input: {}", input);
    println!(" ");
    let ast = parser.parse();
    
    println!("ast : {:?}", ast);
}
pub fn test_evaluator(input: &str){
    let mut parser = Parser::new(input.parse().unwrap());
    let ast = parser.parse();
    println!("Full AST: {:#?}", ast);
    let result = crate::evaluator::evaluate(&ast);
    println!("Evaluation result of '{}': {}", input, result);
}