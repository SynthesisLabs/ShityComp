use crate::lexer::Token;

mod lexer;

fn main() {
    let input = "123+456-200";
    test_lexer(input)
}
fn test_lexer(input: &str){
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
