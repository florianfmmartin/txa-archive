extern crate regex;
use regex::Regex;

pub fn parse(content: String) {
    let tokens = tokenize(content);
    println!("{:#?}", tokens);
}

fn tokenize(content: String) -> Vec<String> {
    let re = Regex::new(r#"(?m)".*"|\$?\w+"#).unwrap();
    let mut tokens = Vec::new();
    for m in re.find_iter(&content) {
        tokens.push(String::from(m.as_str()));
    };
    tokens
}