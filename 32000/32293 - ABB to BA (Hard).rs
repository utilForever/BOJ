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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn convert(stack: &mut Vec<char>) {
    while stack.len() >= 3
        && stack[stack.len() - 3] == 'A'
        && stack[stack.len() - 2] == 'B'
        && stack[stack.len() - 1] == 'B'
    {
        stack.pop();
        stack.pop();
        stack.pop();

        stack.push('B');
        convert(stack);
        stack.push('A');
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let _ = scan.token::<usize>();
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut stack = Vec::new();

        for c in s {
            stack.push(c);
            convert(&mut stack);
        }

        writeln!(out, "{}", stack.iter().collect::<String>()).unwrap();
    }
}
