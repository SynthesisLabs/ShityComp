use crate::lexer;
use crate::lexer::Token;

//file containing all test methods
pub fn test_lexer(input: &str){
    //initialize lexer
    let mut lexer = lexer::Lexer::new(input);
    loop{
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF||token == Token::Err{
            break;
        }
    }

}
