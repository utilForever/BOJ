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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let password = scan.token::<String>().chars().collect::<Vec<_>>();

        if password.len() < 6 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let mut ret = i64::MAX;

        for i in 0..password.len() {
            let mut has_uppercase = false;
            let mut has_lowercase = false;
            let mut has_digit = false;
            let mut len = 0;

            for j in i..password.len() {
                if password[j].is_ascii_uppercase() {
                    has_uppercase = true;
                } else if password[j].is_ascii_lowercase() {
                    has_lowercase = true;
                } else if password[j].is_ascii_digit() {
                    has_digit = true;
                }

                len += 1;

                if has_uppercase && has_lowercase && has_digit {
                    len = len.max(6);
                    ret = ret.min(len);
                    break;
                }
            }
        }

        writeln!(out, "{}", if ret == i64::MAX { 0 } else { ret }).unwrap();
    }
}
