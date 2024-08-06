use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let priority = |operator: char| -> i32 {
        match operator {
            '+' | '-' => 1,
            '*' => 2,
            _ => 0,
        }
    };

    let calculate = |stack: &mut Vec<i64>, operator: char| {
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match operator {
            '+' => stack.push(a | b),
            '-' => stack.push(a & !b),
            '*' => stack.push(a & b),
            _ => (),
        }
    };

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let s = s.chars().collect::<Vec<_>>();
        let mut stack = Vec::new();
        let mut operators = Vec::new();
        let mut idx = 0;

        while idx < s.len() {
            match s[idx] {
                '{' => {
                    let mut val = 0;
                    idx += 1;

                    while s[idx] != '}' {
                        val |= 1 << (s[idx] as u8 - b'A');
                        idx += 1;
                    }

                    stack.push(val);
                    idx -= 1;
                }
                '(' => {
                    operators.push(s[idx]);
                }
                ')' => {
                    while !operators.is_empty() && operators[operators.len() - 1] != '(' {
                        calculate(&mut stack, operators.pop().unwrap());
                    }

                    operators.pop();
                }
                '+' | '-' | '*' => {
                    while !operators.is_empty()
                        && priority(operators[operators.len() - 1]) >= priority(s[idx])
                    {
                        calculate(&mut stack, operators.pop().unwrap());
                    }

                    operators.push(s[idx]);
                }
                _ => (),
            }

            idx += 1;
        }

        while !operators.is_empty() {
            calculate(&mut stack, operators.pop().unwrap());
        }

        write!(out, "{{").unwrap();

        for i in 0..26 {
            if stack[0] & (1 << i) != 0 {
                write!(out, "{}", (b'A' + i as u8) as char).unwrap();
            }
        }

        writeln!(out, "}}").unwrap();
    }
}
