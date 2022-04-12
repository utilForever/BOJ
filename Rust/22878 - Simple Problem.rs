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

fn calculate(arr: Vec<i128>, n: usize) -> i128 {
    let mut ret = 0;

    for i in 0..n {
        ret += (2 * (i + 1) as i128 - 1 - n as i128) * arr[i];
    }

    2 * ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = scan.token::<i128>();
    }

    let mut q = vec![0; n];
    for i in 0..n {
        q[i] = scan.token::<i128>();
    }

    let mut r = vec![0; n];
    for i in 0..n {
        r[i] = p[i] + q[i];
    }

    let mut s = vec![0; n];
    for i in 0..n {
        s[i] = p[i] - q[i];
    }

    p.sort();
    q.sort();
    r.sort();
    s.sort();

    writeln!(
        out,
        "{}",
        calculate(p, n) + calculate(q, n) - ((calculate(r, n) + calculate(s, n)) / 2)
    )
    .unwrap();
}
