use crate::lexer::Token;

mod lexer;

fn main() {
    let input = "123+456-200";
    let mut lexer = lexer::Lexer::new(input);
    loop{
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF||token == Token::Err{
            break;
        }
    }
}
