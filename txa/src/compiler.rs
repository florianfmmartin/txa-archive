use std::collections::BTreeMap;

pub type Definition = Vec<String>;

fn remove_comments(tokens: Vec<String>) -> Vec<String> {
    let mut in_comment: bool = false;
    let mut tokens_without_comments: Vec<String> = Vec::new();
    for token in tokens {
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

    for token in tokens {
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

    let mut labels = BTreeMap::new();

    let mut index = 0;
    while index < code.len() {
        let token = code.get(index).unwrap();
        if token.as_str() == "jump" {
            index += 2;
            continue;
        }
        if token.chars().nth(0) == Some(':') {
            labels.insert(token.to_string(), index);
            code.remove(index);
        }

        index += 1;
    }

    index = 0;
    while index < code.len() {
        let token = code.get(index).unwrap();
        if token.as_str() == "jump" {
            let label = code.get(index + 1).unwrap();
            let label_index = *labels.get(label).unwrap() as i64;
            let element = code.get_mut(index + 1).unwrap();
            let delta: i64 = label_index - (index as i64);
            *element = delta.to_string();
            index += 2;
        } else {
            index += 1;
        }
    }

    code
}

pub fn run(init_tokens: Vec<String>) -> BTreeMap<String, Definition> {
    let mut tokens = init_tokens;
    tokens = remove_comments(tokens);

    let definitions = group_definitions(tokens);
    let mut declarations_map: BTreeMap<String, Definition> = BTreeMap::new();
    for definition in definitions.clone() {
        let varname = definition.get(0).unwrap().to_string();
        declarations_map.insert(varname, make_definition(definition));
    }

    declarations_map
}
