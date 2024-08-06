use io::Write;
use std::{io, str};

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

fn get_priority(c: char) -> i32 {
    match c {
        '(' => 0,
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let expression = scan.token::<String>();
    let mut operators = Vec::new();

    for c in expression.chars() {
        if c >= 'A' && c <= 'Z' {
            write!(out, "{}", c).unwrap();
        } else if c == '(' {
            operators.push(c);
        } else if c == ')' {
            while operators.last() != Some(&'(') {
                write!(out, "{}", operators.pop().unwrap()).unwrap();
            }

            operators.pop();
        } else {
            while !operators.is_empty()
                && get_priority(*operators.last().unwrap()) >= get_priority(c)
            {
                write!(out, "{}", operators.pop().unwrap()).unwrap();
            }

            operators.push(c);
        }
    }

    while !operators.is_empty() {
        write!(out, "{}", operators.pop().unwrap()).unwrap();
    }

    writeln!(out).unwrap();
}
