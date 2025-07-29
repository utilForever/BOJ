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

    let (_, _) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let t = scan.token::<String>();

    let mut zeros_s = Vec::new();
    let mut zeros_t = Vec::new();

    for (i, c) in s.chars().enumerate() {
        if c == '0' {
            zeros_s.push(i);
        }
    }

    for (i, c) in t.chars().enumerate() {
        if c == '0' {
            zeros_t.push(i);
        }
    }

    let mut total = 0;

    for (&pos_s, &pos_t) in zeros_s.iter().zip(zeros_t.iter()) {
        total += (pos_s as i64 - pos_t as i64).abs();
    }

    writeln!(out, "{}", (total * total + (total % 2)) / 2).unwrap();
}
