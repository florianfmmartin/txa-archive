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

pub fn top(stack: &mut Stack) -> Option<Element> {
    stack.pop()
}

fn dup(stack: &mut Stack) {
    let element = match stack.last().expect("No element to dupplicate") {
        Element::Int(i) => m_int(i.clone()),
        Element::Str(s) => m_str(&s.clone()),
    };
    stack.push(element);
}

fn swp(stack: &mut Stack) {
    let b = stack.pop().expect("Empty stack while swapping!");
    let a = stack.pop().expect("Empty stack while swapping!");
    stack.push(b);
    stack.push(a);
}

fn ovr(stack: &mut Stack) {
    let b = match stack
        .get(stack.len() - 2)
        .expect("Empty stack while doing ovr")
    {
        Element::Int(i) => m_int(i.clone()),
        Element::Str(s) => m_str(&s.clone()),
    };
    stack.push(b);
}

fn rot(stack: &mut Stack) {
    let c = stack.pop().expect("Empty stack while swapping!");
    let b = stack.pop().expect("Empty stack while swapping!");
    let a = stack.pop().expect("Empty stack while swapping!");
    stack.push(b);
    stack.push(c);
    stack.push(a);
}

pub fn print(stack: &mut Stack) {
    match stack.pop() {
        Some(e) => match e {
            Element::Int(i) => println!("{}", i),
            Element::Str(s) => println!("{}", s),
        },
        None => println!("Empty stack!"),
    }
}

// Arithmetics
fn add(stack: &mut Stack) {
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

fn sub(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na - nb)),
        (ta, tb) => panic!("Could not subtract {:?} from {:?}", tb, ta),
    }
}

fn mul(stack: &mut Stack) {
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

fn div(stack: &mut Stack) {
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

fn modulo(stack: &mut Stack) {
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

fn and(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) && int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `and` of {:?} with {:?}", ta, tb),
    }
}

fn ior(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) || int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `or` of {:?} with {:?}", ta, tb),
    }
}

fn xor(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(bool_to_int(
            int_to_bool(na) != int_to_bool(nb),
        ))),
        (ta, tb) => panic!("Could not do logic `xor` of {:?} with {:?}", ta, tb),
    }
}

fn not(stack: &mut Stack) {
    let a = stack.pop();
    match a {
        Some(Element::Int(na)) => stack.push(Element::Int(bool_to_int(!int_to_bool(na)))),
        ta => panic!("Could not do logic `not` of {:?}", ta),
    }
}

// Comparison
fn equ(stack: &mut Stack) {
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

fn neq(stack: &mut Stack) {
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

fn lst(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na < nb)))
        }
        (ta, tb) => panic!("Could not check if {:?} is lesser than {:?}", ta, tb),
    }
}

fn grt(stack: &mut Stack) {
    let b = stack.pop();
    let a = stack.pop();
    match (a, b) {
        (Some(Element::Int(na)), Some(Element::Int(nb))) => {
            stack.push(Element::Int(bool_to_int(na > nb)))
        }
        (ta, tb) => panic!("Could not check if {:?} is lesser than {:?}", ta, tb),
    }
}

pub fn execute(stack: &mut Stack, token: &String) -> bool {
    match token.as_str() {
        "pop" => {
            pop(stack);
            true
        }
        "dup" => {
            dup(stack);
            true
        }
        "swp" => {
            swp(stack);
            true
        }
        "ovr" => {
            ovr(stack);
            true
        }
        "rot" => {
            rot(stack);
            true
        }
        "print" => {
            print(stack);
            true
        }
        "add" => {
            add(stack);
            true
        }
        "sub" => {
            sub(stack);
            true
        }
        "mul" => {
            mul(stack);
            true
        }
        "div" => {
            div(stack);
            true
        }
        "mod" => {
            modulo(stack);
            true
        }
        "and" => {
            and(stack);
            true
        }
        "ior" => {
            ior(stack);
            true
        }
        "xor" => {
            xor(stack);
            true
        }
        "not" => {
            not(stack);
            true
        }
        "equ" => {
            equ(stack);
            true
        }
        "neq" => {
            neq(stack);
            true
        }
        "lst" => {
            lst(stack);
            true
        }
        "grt" => {
            grt(stack);
            true
        }
        _ => false,
    }
}

// Return stack
fn transfer(stack_a: &mut Stack, stack_b: &mut Stack) {
    let element = stack_a.pop().expect("Can't pop from empty stack");
    stack_b.push(match element {
        Element::Int(i) => m_int(i.clone()),
        Element::Str(s) => m_str(&s.clone()),
    });
}

fn copy_onto(stack_a: &mut Stack, stack_b: &mut Stack) {
    let element = stack_a.last().expect("Can't pop from empty stack");
    stack_b.push(match element {
        Element::Int(i) => m_int(i.clone()),
        Element::Str(s) => m_str(&s.clone()),
    });
}

fn rpop(ret: &mut Stack) {
    ret.pop();
}

pub fn execute_ret(ret: &mut Stack, stack: &mut Stack, token: &String) -> bool {
    match token.as_str() {
        "trs" => {
            transfer(ret, stack);
            true
        }
        "tsr" => {
            transfer(stack, ret);
            true
        }
        "crs" => {
            copy_onto(ret, stack);
            true
        }
        "csr" => {
            copy_onto(stack, ret);
            true
        }
        "rpop" => {
            rpop(ret);
            true
        }
        _ => false
    }
}
