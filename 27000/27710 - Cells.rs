use io::Write;
use std::{collections::HashMap, io, str};

fn evaluate_expression(
    cell: String,
    expressions: &HashMap<String, String>,
    values: &mut HashMap<String, i64>,
) -> i64 {
    // If the value is already computed, return it
    if values.contains_key(&cell) {
        return values[&cell];
    }

    // Parse the expression and store the value
    let value = parse_expression(expressions[&cell].clone(), expressions, values);
    values.insert(cell.clone(), value);

    value
}

fn parse_expression(
    expression: String,
    expressions: &HashMap<String, String>,
    values: &mut HashMap<String, i64>,
) -> i64 {
    // If the expression doesn't contain any whitespace, it's a number or a variable
    if expression.chars().all(|c| c != ' ') {
        match expression.parse::<i64>() {
            Ok(value) => return value,
            Err(_) => return evaluate_expression(expression, expressions, values),
        }
    }

    let mut pos_operators = [None; 4];

    // Find the position of the operators (rightmost first)
    for (idx, operator) in ['+', '-', '*', '/'].iter().enumerate() {
        if let Some(pos) = expression.chars().rev().position(|c| c == *operator) {
            pos_operators[idx] = Some(expression.len() - pos - 1);
        }
    }

    // Consider operators with higher precedence and left associativity
    // Why use max()? Because we have to process the operators from left to right
    let mut pos = match (pos_operators[0], pos_operators[1]) {
        (Some(a), Some(b)) => Some(a.max(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    if pos.is_none() {
        pos = match (pos_operators[2], pos_operators[3]) {
            (Some(a), Some(b)) => Some(a.max(b)),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        };
    }

    // Consider whitespace
    let pos = match pos {
        Some(pos) => pos,
        None => expression.len() - 1,
    };
    let left = expression[..pos - 1].to_string();
    let right = expression[pos + 2..].to_string();

    // Evaluate the left and right expressions
    let left_value = parse_expression(left, expressions, values);
    let right_value = parse_expression(right, expressions, values);

    // Evaluate the expression
    match expression.chars().nth(pos).unwrap() {
        '+' => left_value + right_value,
        '-' => left_value - right_value,
        '*' => left_value * right_value,
        '/' => left_value / right_value,
        _ => unreachable!(),
    }
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let t = s.parse::<i64>().unwrap();

    for _ in 0..t {
        io::stdin().read_line(&mut s).unwrap();

        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let n = s.parse::<i64>().unwrap();
        let mut expressions = HashMap::new();

        for _ in 0..n {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();

            let splitted = s.split(" = ").collect::<Vec<&str>>();
            let (variable, expression) = (splitted[0], splitted[1]);
            expressions.insert(variable.to_string(), expression.to_string());
        }

        let mut values = HashMap::new();
        let mut ret = Vec::new();

        for (variable, _) in expressions.iter() {
            let value = evaluate_expression(variable.clone(), &expressions, &mut values);
            ret.push((variable, value));
        }

        ret.sort_by(|a, b| a.0.cmp(&b.0));

        for (variable, value) in ret {
            writeln!(out, "{variable} = {value}").unwrap();
        }

        writeln!(out).unwrap();
    }
}
