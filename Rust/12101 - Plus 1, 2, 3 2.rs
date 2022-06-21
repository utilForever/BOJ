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

fn process_expressions(
    ret: &mut Vec<String>,
    num: usize,
    n: usize,
    idx: usize,
    m: usize,
    expression: String,
) {
    if idx == m {
        if num == n {
            ret.push(expression[1..].to_string());
        }

        return;
    }

    process_expressions(ret, num + 1, n, idx + 1, m, expression.clone() + "+1");
    process_expressions(ret, num + 2, n, idx + 1, m, expression.clone() + "+2");
    process_expressions(ret, num + 3, n, idx + 1, m, expression + "+3");
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = Vec::new();

    for i in 1..=n {
        process_expressions(&mut ret, 0, n, 0, i, String::new());
    }

    ret.sort();

    if ret.len() >= k {
        writeln!(out, "{}", ret[k - 1]).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
