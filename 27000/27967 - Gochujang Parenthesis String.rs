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

fn backtrack(
    parentheses: &mut Vec<char>,
    ret: &mut Option<String>,
    idx: usize,
    cnt: i64,
    n: usize,
) {
    if ret.is_some() {
        return;
    }

    if idx == n {
        if cnt == 0 {
            *ret = Some(parentheses.iter().collect());
        }

        return;
    }

    if parentheses[idx] == '(' {
        backtrack(parentheses, ret, idx + 1, cnt + 1, n);
    } else if parentheses[idx] == ')' {
        if cnt >= 1 {
            backtrack(parentheses, ret, idx + 1, cnt - 1, n);
        }
    } else {
        parentheses[idx] = '(';
        backtrack(parentheses, ret, idx + 1, cnt + 1, n);
        parentheses[idx] = 'G';

        if cnt >= 1 {
            parentheses[idx] = ')';
            backtrack(parentheses, ret, idx + 1, cnt - 1, n);
            parentheses[idx] = 'G';
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut ret = None;

    backtrack(&mut s, &mut ret, 0, 0, n);

    writeln!(out, "{}", ret.unwrap()).unwrap();
}
