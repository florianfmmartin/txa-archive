mod compiler;
mod interpreter;
mod stack;
mod tokenizer;

fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args.get(1).expect("No filename given");
    std::fs::read_to_string(filename).expect("Could not get content from file")
}

fn main() {
    let debug = false;
    let file_content = read_file_from_args();
    let tokens = tokenizer::tokenize(file_content);
    let declarations = compiler::run(tokens);
    interpreter::run(declarations, debug);
}
