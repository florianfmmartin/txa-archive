use super::compiler::Definition;
use super::stack;
use std::collections::BTreeMap;

struct Scope {
    name: String,
    index: usize,
    vars: BTreeMap<String, stack::Element>,
    parent: Option<Box<Scope>>,
}

fn enter_scope(scope: Scope, scope_name: String) -> Scope {
    Scope {
        name: scope_name,
        index: 0,
        vars: BTreeMap::new(),
        parent: Some(Box::new(scope)),
    }
}

fn leave_scope(scope: Scope) -> Scope {
    match scope.parent {
        Some(s) => *s,
        None => panic!("Cannot leave root scope"),
    }
}

pub fn run(declarations: BTreeMap<String, Definition>, debug: bool) {
    let mut scope = Scope {
        name: String::from("$main"),
        index: 0,
        vars: BTreeMap::new(),
        parent: None,
    };

    let mut stack: stack::Stack = Vec::new();

    let mut halt = false;

    while !halt {
        let declaration = declarations
            .get(&scope.name)
            .expect("Could not find declartion with given name");
        let token = declaration.code.get(scope.index).expect("Cannot get token");

        if debug {
            if token.as_str() == "local" || token.as_str() == "jump" {
                let next = declaration
                    .code
                    .get(scope.index + 1)
                    .expect("Cannot get token");
                println!("{:?} : {:?} -> {:?}", token, next, stack);
            } else {
                println!("{:?} -> {:?}", token, stack);
            }
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        if token.chars().nth(0) == Some('$') {
            scope = enter_scope(scope, token.to_string());
            continue;
        }

        if token.as_str() == "endef" {
            if scope.name == "$main" {
                halt = true;
            } else {
                scope = leave_scope(scope);
                scope.index += 1;
            }
            continue;
        }

        if token.as_str() == "jump" {
            scope.index += 1;
            let label = declaration
                .code
                .get(scope.index)
                .expect("Cannot get label for jump");
            let jump_index = declaration
                .labels
                .get(label)
                .expect("Do not know this label");
            let should_jump = match stack::top(&mut stack) {
                Some(stack::Element::Int(i)) => {
                    if i != 0 {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if should_jump {
                scope.index = *jump_index;
            } else {
                scope.index += 1;
            }
            continue;
        }

        if token.chars().nth(0) == Some(':') {
            scope.index += 1;
            continue;
        }

        if token.as_str() == "local" {
            scope.index += 1;
            let varname = declaration
                .code
                .get(scope.index)
                .expect("Cannot get var name");
            let value = stack::top(&mut stack).expect("No value to put into var");
            scope.vars.insert(varname.to_string(), value);
            scope.index += 1;
            continue;
        }

        if token.chars().nth(0) == Some('@') {
            match scope.vars.get(token) {
                Some(t) => match t {
                    stack::Element::Int(i) => stack::push(&mut stack, stack::m_int(*i)),
                    stack::Element::Str(s) => stack::push(&mut stack, stack::m_str(s)),
                },
                None => panic!("Variable {:?} used before being defined", token),
            }
            scope.index += 1;
            continue;
        }

        if token.chars().nth(0) == Some('"') && token.chars().nth_back(0) == Some('"') {
            stack::push(&mut stack, stack::m_str(token));
            scope.index += 1;
            continue;
        }

        let try_int = token.parse::<i64>();
        match try_int {
            Ok(i) => {
                stack::push(&mut stack, stack::m_int(i));
                scope.index += 1;
                continue;
            }
            Err(_) => (),
        };

        let try_stack = stack::execute(&mut stack, token);
        if try_stack {
            scope.index += 1;
            continue;
        }

        // Did not hit anything
        halt = true;
    }
}
