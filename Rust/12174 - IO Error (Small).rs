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

    for i in 1..=t {
        let b = scan.token::<i64>();
        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();
        let mut ret = String::new();

        for j in 0..b {
            let mut multiplier = 1;
            let mut val = 0;

            for k in (0..8).rev() {
                val += multiplier * if s[(j * 8 + k) as usize] == 'I' { 1 } else { 0 };
                multiplier *= 2;
            }

            ret.push(val as u8 as char);
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
