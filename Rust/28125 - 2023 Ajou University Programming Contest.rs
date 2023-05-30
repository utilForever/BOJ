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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();
        let mut ret = String::new();
        let mut idx = 0;
        let mut cnt = 0;

        while idx < s.len() {
            if s[idx] == '@' {
                ret.push('a');
                cnt += 1;
            } else if s[idx] == '[' {
                ret.push('c');
                cnt += 1;
            } else if s[idx] == '!' {
                ret.push('i');
                cnt += 1;
            } else if s[idx] == ';' {
                ret.push('j');
                cnt += 1;
            } else if s[idx] == '^' {
                ret.push('n');
                cnt += 1;
            } else if s[idx] == '0' {
                ret.push('o');
                cnt += 1;
            } else if s[idx] == '7' {
                ret.push('t');
                cnt += 1;
            } else if s[idx] == '\\' && s[idx + 1] == '\'' {
                ret.push('v');
                idx += 1;
                cnt += 1;
            } else if s[idx] == '\\' && s[idx + 1] == '\\' && s[idx + 2] == '\'' {
                ret.push('w');
                idx += 2;
                cnt += 1;
            } else {
                ret.push(s[idx]);
            }

            idx += 1;
        }

        if cnt * 2 >= ret.len() {
            writeln!(out, "I don't understand").unwrap();
        } else {
            writeln!(out, "{ret}").unwrap();
        }
    }
}
