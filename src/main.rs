
mod lexer;
mod test;
mod parser;

fn main() {
    let input = "123 231 333";
    
    test::test_parser(input);
}
