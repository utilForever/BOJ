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
        let s = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut idx = 0;

        if s[idx] != 'A' {
            idx += 1;
        }

        if idx >= s.len() || s[idx] != 'A' {
            writeln!(out, "Good").unwrap();
            continue;
        }

        while idx < s.len() && s[idx] == 'A' {
            idx += 1;
        }

        if idx >= s.len() || s[idx] != 'F' {
            writeln!(out, "Good").unwrap();
            continue;
        }

        while idx < s.len() && s[idx] == 'F' {
            idx += 1;
        }

        if idx >= s.len() || s[idx] != 'C' {
            writeln!(out, "Good").unwrap();
            continue;
        }

        while idx < s.len() && s[idx] == 'C' {
            idx += 1;
        }

        if idx >= s.len() {
            writeln!(out, "Infected!").unwrap();
            continue;
        }

        if s[idx] == 'A'
            || s[idx] == 'B'
            || s[idx] == 'C'
            || s[idx] == 'D'
            || s[idx] == 'E'
            || s[idx] == 'F'
        {
            idx += 1;
        } else {
            writeln!(out, "Good").unwrap();
            continue;
        }

        writeln!(out, "{}", if idx < s.len() { "Good" } else { "Infected!" }).unwrap();
    }
}
