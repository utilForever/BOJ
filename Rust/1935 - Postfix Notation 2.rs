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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let expression = scan.token::<String>();
    let mut values = vec![0; n];

    for i in 0..n {
        values[i] = scan.token::<i64>();
    }

    let mut stack = Vec::new();

    for ch in expression.chars() {
        if ch >= 'A' && ch <= 'Z' {
            stack.push(values[(ch as u8 - 'A' as u8) as usize] as f64);
        } else if ch == '+' {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a + b);
        } else if ch == '-' {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a - b);
        } else if ch == '*' {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a * b);
        } else if ch == '/' {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(a / b);
        }
    }

    let ret = stack.pop().unwrap();
    writeln!(out, "{:.2}", ret).unwrap();
}
