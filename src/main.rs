
mod lexer;
mod test;
mod parser;
mod evaluator;
mod file_reader;

fn main() {
    let input;
    match file_reader::read_file_to_string("./calc.yor"){
        Ok(contents)=>{
            input = contents;
        }
        Err(e)=>{
            eprintln!("Error reading file: {}", e);
            return;
        }
    }
    test::test_evaluator(&*input);
}
