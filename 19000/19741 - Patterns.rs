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

fn divisors(n: i64) -> Vec<i64> {
    let mut ret = Vec::new();

    for i in 2..=(n as f64).sqrt() as i64 {
        if n % i == 0 {
            ret.push(i);

            if i * i != n {
                ret.push(n / i);
            }
        }
    }

    if n > 1 {
        ret.push(1);
    }
    
    ret.push(n);

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ret = vec![vec!['.'; n]; n];

    for i in 0..n {
        for j in 0..n {
            let val = i * n + (j + 1);

            if divisors(val as i64).len() <= k {
                ret[i][j] = '*';
            }
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
