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
        let (day, month, year) = (
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );

        if day == 0 && month == 0 && year == 0 {
            break;
        }

        let line = scan.line().trim().to_string();
        let s = ((day + month + year) % 25 + 1) as u8;

        for c in line.chars() {
            if c.is_ascii_alphabetic() {
                write!(out, "{}", ((c as u8 - b'a' + 26 - s) % 26 + b'a') as char).unwrap();
            } else {
                write!(out, "{c}").unwrap();
            }
        }

        writeln!(out).unwrap();
    }
}
