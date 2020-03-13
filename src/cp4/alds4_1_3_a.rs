use std::num::ParseIntError;

struct Stack {
    data: Vec<i32>,
    top: usize,
    max: usize,
}

impl Stack {
    fn new(size: usize) -> Stack {
        Stack {
            data: vec![0; size],
            top: 0,
            max: size,
        }
    }
    fn is_empty(&self) -> bool {
        self.top == 0
    }
    fn is_full(&self) -> bool {
        self.top >= self.max - 1
    }
    fn push(&mut self, x: i32) {
        if self.is_full() {
            panic!("stack overflow");
        }
        self.top += 1;
        self.data[self.top] = x;
    }
    fn pop(&mut self) -> i32 {
        if self.is_empty() {
            panic!("stack underflow")
        }
        self.top -= 1;
        self.data[self.top + 1]
    }
}

pub fn run(s: String) -> i32 {
    let sl: Vec<&str> = s.split_whitespace().collect();
    let mut stack = Stack::new(1000);

    for i in 0..sl.len() {
        match parse_i32(sl[i]) {
            Ok(n) => {
                stack.push(n);
            }
            Err(_) => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(calc(b, a, sl[i]));
            }
        }
    }

    stack.pop()
}

fn calc(a: i32, b: i32, operator: &str) -> i32 {
    match operator {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        _ => panic!("The operator must be `+`, `-`, or `*`."),
    }
}

fn parse_i32(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}
