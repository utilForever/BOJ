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

    let len = s.len();
    let mut idx = 0;
    let mut ret = 0;

    while idx < s.len() {
        if idx + 1 < len && s[idx] == 'c' && (s[idx + 1] == '=' || s[idx + 1] == '-') {
            idx += 2;
        } else if idx + 2 < len && s[idx] == 'd' && s[idx + 1] == 'z' && s[idx + 2] == '=' {
            idx += 3;
        } else if idx + 1 < len && s[idx] == 'd' && s[idx + 1] == '-' {
            idx += 2;
        } else if idx + 1 < len && s[idx] == 'l' && s[idx + 1] == 'j' {
            idx += 2;
        } else if idx + 1 < len && s[idx] == 'n' && s[idx + 1] == 'j' {
            idx += 2;
        } else if idx + 1 < len && s[idx] == 's' && s[idx + 1] == '=' {
            idx += 2;
        } else if idx + 1 < len && s[idx] == 'z' && s[idx + 1] == '=' {
            idx += 2;
        } else {
            idx += 1;
        }

        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
