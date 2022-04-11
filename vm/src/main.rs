mod vm {
    #[derive(Debug)]
    pub enum Element {
        Int(i64),
        Str(String),
    }

    pub fn m_int(i: i64) -> Element {
        Element::Int(i)
    }

    pub fn m_str(s: &str) -> Element {
        Element::Str(s.to_string())
    }

    pub type Stack = Vec<Element>;

    pub fn push(stack: &mut Stack, value: Element) {
        stack.push(value);
    }

    pub fn pop(stack: &mut Stack) {
        stack.pop();
    }

    pub fn print(stack: &Stack) {
        print!("{:?}\n", stack);
    }

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
            (ta, tb) => panic!("\n\nError:\nI could not add {:?} and {:?}\n\n", ta, tb),
        }
    }

    pub fn sub(stack: &mut Stack) {
        let b = stack.pop();
        let a = stack.pop();
        match (a, b) {
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na - nb)),
            (ta, tb) => panic!(
                "\n\nError:\nI could not subtract {:?} from {:?}\n\n",
                tb, ta
            ),
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
            (ta, tb) => panic!("\n\nError:\nI could not multiply {:?} by {:?}\n\n", ta, tb),
        }
    }

    pub fn div(stack: &mut Stack) {
        let b = stack.pop();
        let a = stack.pop();
        match (a, b) {
            (Some(Element::Int(na)), Some(Element::Int(0))) => {
                panic!("\n\nError:\nI could not divide {:?} by zero\n\n", na)
            }
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na / nb)),
            (ta, tb) => panic!("\n\nError:\nI could not divide {:?} by {:?}\n\n", ta, tb),
        }
    }

    pub fn modulo(stack: &mut Stack) {
        let b = stack.pop();
        let a = stack.pop();
        match (a, b) {
            (Some(Element::Int(na)), Some(Element::Int(0))) => {
                panic!("\n\nError:\nI could not divide {:?} by zero\n\n", na)
            }
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(na % nb)),
            (ta, tb) => panic!("\n\nError:\nI could not modulo {:?} by {:?}\n\n", ta, tb),
        }
    }

    pub fn int_to_bool(int: i64) -> bool {
        if int > 0 {
            true
        } else {
            false
        }
    }

    pub fn bool_to_int(b: bool) -> i64 {
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
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(
                bool_to_int(int_to_bool(na) && int_to_bool(nb)),
            )),
            (ta, tb) => panic!("\n\nError:\nI could not do logic `and` of {:?} by {:?}\n\n", ta, tb),
        }
    }

    pub fn ior(stack: &mut Stack) {
        let b = stack.pop();
        let a = stack.pop();
        match (a, b) {
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(
                bool_to_int(int_to_bool(na) || int_to_bool(nb)),
            )),
            (ta, tb) => panic!("\n\nError:\nI could not do logic `or` of {:?} by {:?}\n\n", ta, tb),
        }
    }

    pub fn xor(stack: &mut Stack) {
        let b = stack.pop();
        let a = stack.pop();
        match (a, b) {
            (Some(Element::Int(na)), Some(Element::Int(nb))) => stack.push(Element::Int(
                bool_to_int(int_to_bool(na) != int_to_bool(nb)),
            )),
            (ta, tb) => panic!("\n\nError:\nI could not do logic `xor` of {:?} by {:?}\n\n", ta, tb),
        }
    }

    /*
     * equ
     * neq
     * not
     * lst
     * grt
     * cnd
     * whl
     * end
     */
}

fn main() {
    let mut stack: vm::Stack = Vec::new();

    vm::push(&mut stack, vm::m_int(13));
    vm::push(&mut stack, vm::m_int(4));
    vm::and(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);

    vm::push(&mut stack, vm::m_int(13));
    vm::push(&mut stack, vm::m_int(0));
    vm::and(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);

    vm::push(&mut stack, vm::m_int(-6));
    vm::push(&mut stack, vm::m_int(0));
    vm::and(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);
}
