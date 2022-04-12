use super::stack;
use std::collections::HashMap;

struct Scope {
    code: Vec<String>,
    vars: HashMap<String, stack::Element>,
    labels: HashMap<String, i64>,
}

fn remove_comments(tokens: Vec<String>) -> Vec<String> {
    let mut in_comment: bool = false;
    let mut tokens_without_comments: Vec<String> = Vec::new();
    for token in tokens.into_iter() {
        if !in_comment {
            match token.as_str() {
                "#[" => in_comment = true,
                t => tokens_without_comments.push(t.to_string()),
            }
        } else {
            if token.as_str() == "]#" {
                in_comment = false;
            }
        }
    }

    println!("{:?}", tokens_without_comments);
    tokens_without_comments
}

pub fn run(init_tokens: Vec<String>) {
    let mut tokens = init_tokens;

    let mut stack: stack::Stack = Vec::new();
    let mut declaration_map: HashMap<String, Scope> = HashMap::new();

    let mut ignore_next: bool = false;
    let mut in_definition: bool = false;
    let mut current_definition: String = String::from("");

    tokens = remove_comments(tokens)
}
