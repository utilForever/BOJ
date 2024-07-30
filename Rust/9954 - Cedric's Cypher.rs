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

    loop {
        let s = scan.line().trim().to_string();

        if s == "#" {
            break;
        }

        let shift = s.chars().last().unwrap() as u8 - b'A';

        for c in s.chars().take(s.len() - 1) {
            if !c.is_ascii_alphabetic() {
                write!(out, "{c}").unwrap();
            } else {
                let shifted = (c as u8) - shift;
                let shifted = if (c.is_ascii_uppercase() && shifted < b'A')
                    || (c.is_ascii_lowercase() && shifted < b'a')
                {
                    shifted + 26
                } else {
                    shifted
                };

                write!(out, "{}", shifted as char).unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
