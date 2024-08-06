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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut stack = Vec::new();
    let mut multipiler = 1;
    let mut ret = 0;

    for i in 0..s.len() {
        match s[i] {
            '(' => {
                stack.push('(');
                multipiler *= 2;
            }
            '[' => {
                stack.push('[');
                multipiler *= 3;
            }
            ')' => {
                if stack.is_empty() || *stack.last().unwrap() != '(' {
                    writeln!(out, "0").unwrap();
                    return;
                }

                if s[i - 1] == '(' {
                    ret += multipiler;
                }

                stack.pop();
                multipiler /= 2;
            }
            ']' => {
                if stack.is_empty() || *stack.last().unwrap() != '[' {
                    writeln!(out, "0").unwrap();
                    return;
                }

                if s[i - 1] == '[' {
                    ret += multipiler;
                }

                stack.pop();
                multipiler /= 3;
            }
            _ => unreachable!(),
        }
    }

    writeln!(out, "{}", if stack.is_empty() { ret } else { 0 }).unwrap();
}
