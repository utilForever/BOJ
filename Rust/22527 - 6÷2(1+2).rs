use io::Write;
use std::{
    collections::{HashMap, HashSet},
    io, str,
};

pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn process_parentheses(expression: &[char], mut idx: usize) -> usize {
    let mut cnt = 0;

    loop {
        if expression[idx] == '(' {
            cnt += 1;
        } else if expression[idx] == ')' {
            cnt -= 1;
        }

        if cnt == 0 {
            return idx;
        }

        idx += 1;
    }
}

fn parse_expression(
    expression_list: &mut HashMap<Vec<char>, HashSet<i64>>,
    expression: Vec<char>,
) -> HashSet<i64> {
    // Check if expression is already parsed
    if let Some(num_cases) = expression_list.get(&expression) {
        return num_cases.clone();
    }

    // Check if expression is a number
    if expression.to_vec().iter().all(|&c| c.is_digit(10)) {
        let num = expression
            .iter()
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        expression_list.insert(expression.clone(), vec![num].iter().cloned().collect());

        return vec![num].iter().cloned().collect();
    }

    // Check if expression is in parenthesis
    // If so, remove parenthesis and parse expression
    if expression.first() == Some(&'(') && process_parentheses(&expression, 0) == expression.len() - 1
    {
        let ret = parse_expression(
            expression_list,
            expression[1..expression.len() - 1].to_vec(),
        );
        expression_list.insert(expression.clone(), ret.clone());

        return ret;
    }

    let mut ret = HashSet::new();
    let mut idx = 0;

    while idx < expression.len() {
        // Skip expression in parenthesis
        if expression[idx] == '(' {
            idx = process_parentheses(&expression, idx);
        }

        // Process <expression> <operator> <expression>
        if expression[idx] == '+'
            || expression[idx] == '-'
            || expression[idx] == '*'
            || expression[idx] == '/'
        {
            let expression_left = parse_expression(expression_list, expression[..idx].to_vec());
            let expression_right =
                parse_expression(expression_list, expression[idx + 1..].to_vec());

            for &val_left in expression_left.iter() {
                for &val_right in expression_right.iter() {
                    match expression[idx] {
                        '+' => {
                            ret.insert(val_left + val_right);
                        }
                        '-' => {
                            ret.insert(val_left - val_right);
                        }
                        '*' => {
                            ret.insert(val_left * val_right);
                        }
                        '/' => {
                            // Check if division by zero
                            if val_right != 0 {
                                ret.insert(val_left / val_right);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        idx += 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let expression = scan.token::<String>();

        if expression == "#" {
            break;
        }

        let mut expression_list = HashMap::new();
        let ret = parse_expression(&mut expression_list, expression.chars().collect());

        writeln!(out, "{}", ret.len()).unwrap();
    }
}
