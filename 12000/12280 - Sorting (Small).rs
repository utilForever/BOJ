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
        let mut flags = vec![false; n];
        let mut odd = Vec::new();
        let mut even = Vec::new();

        for j in 0..n {
            let num = scan.token::<i64>();
            flags[j] = num.abs() % 2 == 1;

            if flags[j] {
                odd.push(num);
            } else {
                even.push(num);
            }
        }

        odd.sort_by(|a, b| b.cmp(a));
        even.sort();

        let mut ret = vec![0; n];

        for j in 0..n {
            ret[j] = if flags[j] {
                odd.pop().unwrap()
            } else {
                even.pop().unwrap()
            };
        }

        write!(out, "Case #{i}: ").unwrap();

        for j in 0..n {
            write!(out, "{} ", ret[j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
