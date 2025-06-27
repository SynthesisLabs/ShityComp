
mod lexer;
mod test;

fn main() {
    let input = "123+456 - 200";
    test::test_lexer(input)
}
