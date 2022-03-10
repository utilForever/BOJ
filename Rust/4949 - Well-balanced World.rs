use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.replace("\r", "");
        s = s.replace("\n", "");

        if s == "." {
            break;
        }

        let mut parenthesis = Vec::new();
        let mut should_pass = false;

        for c in s.chars() {
            if c == '(' || c == '[' {
                parenthesis.push(c);
            } else if c == ')' {
                if parenthesis.is_empty() || *parenthesis.last().unwrap() != '(' {
                    should_pass = true;
                    writeln!(out, "no").unwrap();
                    break;
                } else {
                    parenthesis.pop();
                }
            } else if c == ']' {
                if parenthesis.is_empty() || *parenthesis.last().unwrap() != '[' {
                    should_pass = true;
                    writeln!(out, "no").unwrap();
                    break;
                } else {
                    parenthesis.pop();
                }
            }
        }

        if should_pass {
            continue;
        }

        if parenthesis.is_empty() {
            writeln!(out, "yes").unwrap();
        } else {
            writeln!(out, "no").unwrap();
        }
    }
}
