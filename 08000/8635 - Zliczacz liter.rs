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

    let n = scan.token::<usize>();
    let mut uppercase = [0; 26];
    let mut lowercase = [0; 26];

    for _ in 0..n {
        let line = scan.line().trim().to_string();

        for c in line.chars() {
            if c.is_ascii_uppercase() {
                uppercase[(c as u8 - b'A') as usize] += 1;
            } else if c.is_ascii_lowercase() {
                lowercase[(c as u8 - b'a') as usize] += 1;
            }
        }
    }

    for i in 0..26 {
        if lowercase[i] == 0 {
            continue;
        }

        writeln!(out, "{} {}", (b'a' + i as u8) as char, lowercase[i]).unwrap();
    }

    for i in 0..26 {
        if uppercase[i] == 0 {
            continue;
        }

        writeln!(out, "{} {}", (b'A' + i as u8) as char, uppercase[i]).unwrap();
    }
}
