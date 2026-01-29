
mod lexer;
mod test;
mod parser;
mod evaluator;

fn main() {
    let input = "123+231*333";
    test::test_evaluator(input);
}
