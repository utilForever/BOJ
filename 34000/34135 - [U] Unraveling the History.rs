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

    let t = scan.token::<usize>();
    let mut pool = Vec::new();
    let mut ranges = Vec::with_capacity(t + 1);

    ranges.push((0, 0));

    for i in 1..=t {
        let mut c = scan.token::<i128>();
        let base = 26 + i as i128;
        let mut digits = Vec::new();

        while c != 0 {
            let r = c.rem_euclid(base);

            if r <= 26 {
                digits.push(r as i64);
                c = c.div_euclid(base);
            } else {
                digits.push((r - base) as i64);
                c = c.div_euclid(base) + 1;
            }
        }

        digits.reverse();

        let mut len = 0;

        for &digit in digits.iter() {
            if digit > 0 {
                len += 1;
            } else {
                let idx = (-digit) as usize;
                let (s, e) = ranges[idx];

                len += e - s;
            }
        }

        let mut ret = Vec::with_capacity(len);

        for &digit in digits.iter() {
            if digit > 0 {
                ret.push(b'A' + (digit - 1) as u8);
            } else {
                let idx = (-digit) as usize;
                let (s, e) = ranges[idx];
                ret.extend_from_slice(&pool[s..e]);
            }
        }

        writeln!(out, "{}", String::from_utf8(ret.clone()).unwrap()).unwrap();

        let start = pool.len();
        pool.extend_from_slice(&ret);
        let end = pool.len();

        ranges.push((start, end));
    }
}
