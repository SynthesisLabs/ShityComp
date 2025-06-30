use crate::lexer;
use crate::lexer::Token;
use crate::parser::Parser;

//file containing all test methods
pub fn test_lexer(input: &str){
    //initialize lexer
    let mut lexer = lexer::Lexer::new(input);
    loop{
        let token = lexer.next_token();
        println!("{:?} \n", token);
        if token == Token::EOF||token == Token::Err{
            break;
        }
        lexer.tokens.push(token);
    }
    println!("Token list:  {:?}", lexer.tokens);
}
pub fn test_parser(input: &str){
    let mut parser = Parser::new(input);
    let ast = parser.parse();
    println!("ast : {:?}", ast);
}