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

    let n = scan.token::<i64>();
    let mut deque = VecDeque::new();

    for _ in 0..n {
        let command = scan.token::<String>();

        match command.as_str() {
            "push_front" => {
                let x = scan.token::<i64>();
                deque.push_front(x);
            }
            "push_back" => {
                let x = scan.token::<i64>();
                deque.push_back(x);
            }
            "pop_front" => {
                writeln!(out, "{}", deque.pop_front().unwrap_or(-1)).unwrap();
            }
            "pop_back" => {
                writeln!(out, "{}", deque.pop_back().unwrap_or(-1)).unwrap();
            }
            "size" => {
                writeln!(out, "{}", deque.len()).unwrap();
            }
            "empty" => {
                writeln!(out, "{}", if deque.is_empty() { 1 } else { 0 }).unwrap();
            }
            "front" => {
                writeln!(out, "{}", deque.front().unwrap_or(&-1)).unwrap();
            }
            "back" => {
                writeln!(out, "{}", deque.back().unwrap_or(&-1)).unwrap();
            }
            _ => unreachable!(),
        }
    }
}
