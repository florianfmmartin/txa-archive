mod vm {
    #[derive(Debug)]
    pub enum Element {
        Int(i64),
        Str(String),
    }

    pub fn mInt(i: i64) -> Element {
        Element::Int(i)
    }

    pub fn mStr(s: &str) -> Element {
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

    /*
     * div
     * and
     * ior
     * xor
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

    vm::push(&mut stack, vm::mStr("hello, "));
    vm::push(&mut stack, vm::mStr("world!"));
    vm::push(&mut stack, vm::mStr("world!"));
    vm::pop(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);
    vm::pop(&mut stack);

    vm::push(&mut stack, vm::mInt(3));
    vm::push(&mut stack, vm::mInt(4));
    vm::mul(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);

    // vm::push(&mut stack, vm::mStr("hello, "));
    // vm::push(&mut stack, vm::mStr("world!"));
    // vm::mul(&mut stack);
    // vm::print(&stack);
    // vm::pop(&mut stack);

    vm::push(&mut stack, vm::mStr("world!"));
    vm::push(&mut stack, vm::mInt(3));
    vm::mul(&mut stack);
    vm::print(&stack);
    vm::pop(&mut stack);
}
