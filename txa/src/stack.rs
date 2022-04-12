// `Element` is the type of data on the `Stack`
#[derive(Debug)]
pub enum Element {
    Int(i64),
    Str(String),
}

// Main type to represent the `Stack`
pub type Stack = Vec<Element>;

// Helper functions to make `Elements` quickly
pub fn m_int(i: i64) -> Element {
    Element::Int(i)
}

pub fn m_str(s: &str) -> Element {
    Element::Str(s.to_string())
}

// Stack operations
pub fn push(stack: &mut Stack, value: Element) {
    stack.push(value);
}

pub fn pop(stack: &mut Stack) {
    stack.pop();
}

pub fn print(stack: &mut Stack) {
    match stack.pop() {
        Some(e) => println!("{:#?}", e),
        None => println!("Empty stack!"),
    }
}

// Arithmetics
pub fn add(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na + nb)),
        (Some(Element::Str(sa)), Some(Element::Str(sb))) => {
            let mut s = String::new();
            s.push_str(&sa);
            s.push_str(&sb);
            stack.push(Element::Str(s));
        }
        (ta, tb) => panic!("Could not add {:?} and {:?}", ta, tb),
    }
}

pub fn sub(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na - nb)),
        (ta, tb) => panic!("Could not subtract {:?} from {:?}", tb, ta),
    }
}

pub fn mul(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na * nb)),
        (Some(Element::Str(sa)), Some(Element::Int(nb))) => {
            stack.push(Element::Str(sa.repeat(nb.try_into().unwrap())))
        }
        (ta, tb) => panic!("Could not multiply {:?} by {:?}", ta, tb),
    }
}

pub fn div(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(0))) => {
            panic!("Could not divide {:?} by zero", na)
        }
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na / nb)),
        (ta, tb) => panic!("Could not divide {:?} by {:?}", ta, tb),
    }
}

pub fn modulo(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(0))) => {
            panic!("Could not divide {:?} by zero", na)
        }
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na % nb)),
        (ta, tb) => panic!("Could not modulo {:?} by {:?}", ta, tb),
    }
}

// Bools
fn int_to_bool(int: i64) -> bool {
    if int > 0 {
        true
    } else {
        false
    }
}

fn bool_to_int(b: bool) -> i64 {
    if b {
        1
    } else {
        0
    }
}

pub fn and(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) && int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `and` of {:?} with {:?}", ta, tb),
    }
}

pub fn ior(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) || int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `or` of {:?} with {:?}", ta, tb),
    }
}

pub fn xor(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) != int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `xor` of {:?} with {:?}", ta, tb),
    }
}

pub fn not(stack: &mut Stack) {
    let a = stack.pop();
    match a {
        Some(Element::Int(na)) => stack.push(Element::Int(bool_to_int(!int_to_bool(na)))),
        ta => panic!("Could not do logic `not` of {:?}", ta),
    }
}

// Comparison
pub fn equ(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na == nb)))
        }
        (Some(Element::Str(sa)), Some(Element::Str(sb))) => {
            stack.push(Element::Int(bool_to_int(sa == sb)))
        }
        (ta, tb) => panic!("Could not check equality of {:?} and {:?}", ta, tb),
    }
}

pub fn neq(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na != nb)))
        }
        (Some(Element::Str(sa)), Some(Element::Str(sb))) => {
            stack.push(Element::Int(bool_to_int(sa != sb)))
        }
        (ta, tb) => panic!("Could not check inequality of {:?} and {:?}", ta, tb),
    }
}

pub fn lst(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na < nb)))
        }
        (ta, tb) => panic!("Could not check if {:?} is lesser than {:?}", ta, tb),
    }
}

pub fn grt(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na > nb)))
        }
        (ta, tb) => panic!("Could not check if {:?} is lesser than {:?}", ta, tb),
    }
}

// To implement (maybe not here even)
/*
 * cnd
 * whl
 * end
 * local
 * define
 */
