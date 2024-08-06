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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let s = scan.token::<String>();
        let mut ret = true;
        let mut idx = 0;

        // Determine s matches regular expression (100+1+ | 01)+
        while idx < s.len() {
            if s[idx..].starts_with("10") {
                idx += 2;

                if idx >= s.len() {
                    ret = false;
                    break;
                }

                if !s[idx..].starts_with("0") {
                    ret = false;
                    break;
                }

                idx += 1;

                while idx < s.len() && s[idx..].starts_with("0") {
                    idx += 1;
                }

                if idx >= s.len() {
                    ret = false;
                    break;
                }

                if !s[idx..].starts_with("1") {
                    ret = false;
                    break;
                }

                idx += 1;

                while idx < s.len() && s[idx..].starts_with("1") {
                    // Check "10"
                    if idx + 1 < s.len() && s[idx + 1..].starts_with("0") {
                        // Check "101"
                        if idx + 2 < s.len() && s[idx + 2..].starts_with("1") {
                            idx += 1;
                        } else {
                            break;
                        }
                    } else {
                        idx += 1;
                    }
                }
            } else if s[idx..].starts_with("01") {
                idx += 2;
            } else {
                ret = false;
                break;
            }
        }

        writeln!(out, "{}", if ret { "YES" } else { "NO" }).unwrap();
    }
}
