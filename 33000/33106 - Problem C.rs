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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let mut s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut idx = 0;

        while idx < s.len() {
            if s[idx] != 'c' {
                idx += 1;
                continue;
            }

            if idx == s.len() - 1 {
                s[idx] = 'k';
                break;
            }

            if s[idx + 1] == 'a'
                || s[idx + 1] == 'o'
                || s[idx + 1] == 'u'
                || (s[idx + 1] != 'e'
                    && s[idx + 1] != 'i'
                    && s[idx + 1] != 'h'
                    && s[idx + 1] != 'y')
            {
                s[idx] = 'k';
            } else if s[idx + 1] == 'e' || s[idx + 1] == 'i' || s[idx + 1] == 'y' {
                s[idx] = 's';
            } else if s[idx + 1] == 'h' {
                s[idx] = 'c';
                s.remove(idx + 1);
            }

            idx += 1;
        }

        writeln!(out, "{}", s.iter().collect::<String>()).unwrap();
    }
}
