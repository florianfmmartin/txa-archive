use std::collections::HashMap;

#[derive(Debug)]
pub struct Definition {
    pub code: Vec<String>,
    pub labels: HashMap<String, usize>,
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

    tokens_without_comments
}

fn group_definitions(tokens: Vec<String>) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut current_definition: Vec<String> = Vec::new();

    for token in tokens.into_iter() {
        match token.as_str() {
            "define" => current_definition.clear(),
            "endef" => {
                current_definition.push(token);
                result.push(current_definition.clone());
            }
            _ => current_definition.push(token),
        }
    }

    result
}

fn make_definition(definition: Vec<String>) -> Definition {
    let mut code = definition.clone();
    code.remove(0);

    let mut labels = HashMap::new();

    let mut index = 0;
    while index < code.len() {
        let token = code.get(index).unwrap();
        if token.as_str() == "jump" {
            index += 2;
            continue;
        }
        if token.starts_with(":") {
            labels.insert(token.to_string(), index);
        }

        index += 1;
    }

    Definition {
        code: code,
        labels: labels,
    }
}

pub fn run(init_tokens: Vec<String>) -> HashMap<String, Definition> {
    let mut tokens = init_tokens;
    tokens = remove_comments(tokens);

    let definitions = group_definitions(tokens);
    let mut declarations_map: HashMap<String, Definition> = HashMap::new();
    for definition in definitions.clone().into_iter() {
        let varname = definition.get(0).unwrap().to_string();
        declarations_map.insert(varname, make_definition(definition));
    }

    declarations_map
}
