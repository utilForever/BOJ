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

static MOD: i32 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut fibonacci = vec![0; n + 1];
    fibonacci[0] = 1;
    fibonacci[1] = 1;

    for i in 2..=n {
        fibonacci[i] = (fibonacci[i - 1] + fibonacci[i - 2]) % MOD;
    }

    let mut queries = vec![(0, 0); q];
    let mut parent = vec![0; n + 1];
    let mut ret = vec![0; n];

    for i in 0..q {
        queries[i] = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
    }

    for (l, r) in queries.iter().rev() {
        let mut idx = *l;

        while idx <= *r {
            if ret[idx] == 0 {
                ret[idx] = fibonacci[idx - *l + 1];
                parent[idx] = *r + 1;
                idx += 1;
            } else {
                idx = parent[idx];
            }
        }
    }

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
