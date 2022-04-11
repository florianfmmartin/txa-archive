mod stack;
mod tokenizer;

fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    let maybe_filename = &args.get(1);
    let filename = match maybe_filename {
        Some(s) => s,
        None => panic!("No filename given")
    };
    let might_get_content = std::fs::read_to_string(filename);
    match might_get_content {
        Ok(s) => s,
        Err(_) => panic!("Could not get content from {:?}", filename),
    }
}

fn main() {
    let file_content = read_file_from_args();
    let tokens = tokenizer::tokenize(file_content);

    let mut stack: stack::Stack = Vec::new();
    stack::push(&mut stack, stack::m_int(4));
}
