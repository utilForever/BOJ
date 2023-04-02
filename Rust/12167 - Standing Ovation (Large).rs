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
        let s_max = scan.token::<usize>();
        let mut shynesses = vec![0; s_max + 1];

        let s = scan.token::<String>();
        let s = s.chars().collect::<Vec<_>>();

        for (i, c) in s.iter().enumerate() {
            shynesses[i] = c.to_digit(10).unwrap() as i64;
        }

        let mut ret = 0;
        let mut person = 0;

        for (idx, &s) in shynesses.iter().enumerate() {
            if person < idx as i64 {
                ret += idx as i64 - person;
                person = idx as i64;
            }

            person += s;
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
