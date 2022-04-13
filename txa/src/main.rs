use clap::Parser;

mod compiler;
mod interpreter;
mod stack;
mod tokenizer;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to run
    filename: String,

    /// Set debug mode to true
    #[clap(long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();
    let debug = args.debug;
    let file_content = std::fs::read_to_string(args.filename).expect("Could not get content from file");
    let tokens = tokenizer::tokenize(file_content);
    let declarations = compiler::run(tokens);
    interpreter::run(declarations, debug);
}
