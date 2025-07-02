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
        if token == Token::EOF{
            return Err("EOF".to_string())
        }else if token == Token::Err{
            panic!("Unexpected Error occurred please try again");
        }else{
            lexer.tokens.push(token);
            return if !lexer.tokens.is_empty(){
                println!("Token list:  {:?}", lexer.tokens);
                Ok(lexer.tokens)
            }else{
                Err("Unexpected error".to_string())
            }
        }

    }
    
}
pub fn test_parser(input: &str){
    let mut parser = Parser::new(input);
    println!(" ");
    println!("Parser input: {}", input);
    println!(" ");
    let ast = parser.parse();
    
    println!("ast : {:?}", ast);
}