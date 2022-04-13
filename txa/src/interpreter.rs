use super::compiler::Definition;
use super::stack;
use std::collections::BTreeMap;

struct Scope {
    name: String,
    index: usize,
    vars: BTreeMap<String, stack::Element>,
    return_stack: stack::Stack,
    parent: Option<Box<Scope>>,
}

fn enter_scope(scope: Scope, scope_name: String) -> Scope {
    Scope {
        name: scope_name,
        index: 0,
        vars: BTreeMap::new(),
        return_stack: Vec::new(),
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
        return_stack: Vec::new(),
        parent: None,
    };

    let mut stack: stack::Stack = Vec::new();

    let mut halt = false;

    let mut declaration = declarations
        .get(&scope.name)
        .expect("Could not find declartion with given name");
    let mut last_scope_name = scope.name.clone();
    while !halt {
        if scope.name != last_scope_name {
            last_scope_name = scope.name.clone();
            declaration = declarations
                .get(&scope.name)
                .expect("Could not find declartion with given name");
        }

        let token = declaration.get(scope.index).expect("Cannot get token");

        if debug {
            if token.as_str() == "local" || token.as_str() == "jump" {
                let next = declaration.get(scope.index + 1).expect("Cannot get token");
                println!(
                    "{:?} : {:?} -> {:?} & {:?}",
                    token, next, stack, scope.return_stack
                );
            } else {
                println!("{:?} -> {:?} & {:?}", token, stack, scope.return_stack);
            }
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }

        if token.chars().next() == Some('$') {
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
            let delta_str = declaration
                .get(scope.index)
                .expect("Cannot get delte for jump");
            let delta: i64 = delta_str.parse::<i64>().unwrap();
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
                let next_index: i64 = (scope.index as i64) + delta - 1;
                if next_index >= 0 {
                    scope.index = next_index as usize;
                } else {
                    panic!("Can't jump to negative index!");
                }
            } else {
                scope.index += 1;
            }
            continue;
        }

        if token.as_str() == "local" {
            scope.index += 1;
            let varname = declaration.get(scope.index).expect("Cannot get var name");
            let value = stack::top(&mut stack).expect("No value to put into var");
            scope.vars.insert(varname.to_string(), value);
            scope.index += 1;
            continue;
        }

        if token.chars().next() == Some('@') {
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

        let chars = &mut token.chars();
        if chars.next() == Some('"') && chars.nth_back(0) == Some('"') {
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

        let try_ret_stack = stack::execute_ret(&mut scope.return_stack, &mut stack, token);
        if try_ret_stack {
            scope.index += 1;
            continue;
        }

        let try_stack = stack::execute(&mut stack, token);
        if try_stack {
            scope.index += 1;
            continue;
        }

        // Did not hit anything
        halt = true;
    }
}
