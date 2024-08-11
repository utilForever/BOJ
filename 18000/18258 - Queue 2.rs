use io::Write;
use std::{collections::VecDeque, io, str};

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

    let mut queue = VecDeque::new();

    let n = scan.token::<usize>();

    for _ in 0..n {
        let order = scan.token::<String>();

        match order.as_str() {
            "push" => {
                let num = scan.token::<i64>();
                queue.push_back(num);
            }
            "pop" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.pop_front().unwrap()).unwrap();
                }
            }
            "size" => {
                writeln!(out, "{}", queue.len()).unwrap();
            }
            "empty" => {
                if queue.is_empty() {
                    writeln!(out, "1").unwrap();
                } else {
                    writeln!(out, "0").unwrap();
                }
            }
            "front" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.front().unwrap()).unwrap();
                }
            }
            "back" => {
                if queue.is_empty() {
                    writeln!(out, "-1").unwrap();
                } else {
                    writeln!(out, "{}", queue.back().unwrap()).unwrap();
                }
            }
            _ => {}
        }
    }
}