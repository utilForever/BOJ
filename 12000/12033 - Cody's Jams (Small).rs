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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut prices = vec![0; 2 * n];

        for j in 0..2 * n {
            prices[j] = scan.token::<i64>();
        }

        let mut ret = Vec::new();

        while !prices.is_empty() {
            let val = prices[0];

            if prices.contains(&(val * 4 / 3)) {
                ret.push(prices[0]);

                let pos = prices.iter().position(|&x| x == val * 4 / 3).unwrap();

                prices.remove(pos);
                prices.remove(0);
            }
        }

        write!(out, "Case #{i}: ").unwrap();

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
