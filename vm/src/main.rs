mod vm {
    #[derive(Debug)]
    pub enum Element {
        Int(i64),
        Str(String),
    }

    pub fn make_int(number: i64) -> Element {
        Element::Int(number)
    }

    pub fn make_str(string: &str) -> Element {
        Element::Str(String::from(string))
    }

    pub type Stack = Vec<Element>;

    pub fn push(stack: &mut Stack, value: Element) {
        stack.push(value);
    }

    pub fn pop(stack: &mut Stack) -> Element {
        match stack.pop() {
            Some(element) => element,
            None => panic!("Popping from empty stack"),
        }
    }

    pub fn print(stack: &Stack) {
        print!("{:?}\n", stack);
    }
}

fn main() {
    let mut stack: vm::Stack = Vec::new();

    vm::push(&mut stack, vm::make_str("hello!"));
    let element = vm::pop(&mut stack);

    match element {
        vm::Element::Int(number) => print!("{}\n", number),
        vm::Element::Str(string) => print!("{}\n", string),
    }

    vm::push(&mut stack, vm::make_str("hello, "));
    vm::push(&mut stack, vm::make_str("world!"));

    vm::print(&stack);
}
