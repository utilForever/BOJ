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

    let n: usize = scan.token();
    let mut num_stack = Vec::new();
    let mut ops = Vec::new();
    let mut num = 1;
    let mut num2 = 0;

    for _ in 0..n {
        let num1 = scan.token();

        if num1 > num2 {
            while num <= num1 {
                num_stack.push(num);
                ops.push('+');

                num += 1;
            }

            if *num_stack.last().unwrap() != num1 {
                writeln!(out, "NO").unwrap();
                return;
            } else {
                num_stack.pop();
                ops.push('-');

                num2 = num1;
            }
        } else {
            if *num_stack.last().unwrap() != num1 {
                writeln!(out, "NO").unwrap();
                return;
            } else {
                num_stack.pop();
                ops.push('-');

                num2 = num1;
            }
        }
    }

    for op in ops.iter() {
        writeln!(out, "{}", op).unwrap();
    }
}
