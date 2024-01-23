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

fn check(s: &str) -> bool {
    let s = s.chars().collect::<Vec<_>>();

    if s.contains(&'*') {
        let mut idx = 0;
        let mut cnt_open = 0;
        let mut cnt_close = 0;

        while s[idx] != '*' {
            if s[idx] == '(' || s[idx] == '?' {
                cnt_open += 1;
            } else {
                cnt_close += 1;
            }

            if cnt_open < cnt_close {
                return false;
            }

            idx += 1;
        }

        cnt_open = 0;
        cnt_close = 0;
        idx = s.len() - 1;

        while s[idx] != '*' {
            if s[idx] == ')' || s[idx] == '?' {
                cnt_close += 1;
            } else {
                cnt_open += 1;
            }

            if cnt_open > cnt_close {
                return false;
            }

            idx -= 1;
        }

        true
    } else {
        if s.len() % 2 != 0 {
            return false;
        }

        let mut cnt_open_orig = s.iter().filter(|&&c| c == '(').count();
        let cnt_close_orig = s.iter().filter(|&&c| c == ')').count();

        if cnt_open_orig > s.len() / 2 || cnt_close_orig > s.len() / 2 {
            return false;
        }

        let mut cnt_open = 0;
        let mut cnt_close = 0;

        for i in 0..s.len() {
            if s[i] == '(' {
                cnt_open += 1;
            } else if s[i] == ')' {
                cnt_close += 1;
            } else {
                if cnt_open_orig < s.len() / 2 {
                    cnt_open_orig += 1;
                    cnt_open += 1;
                } else {
                    cnt_close += 1;
                }
            }

            if cnt_open < cnt_close {
                return false;
            }
        }

        true
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        writeln!(out, "{}", if check(&s) { "YES" } else { "NO" }).ok();
    }
}
