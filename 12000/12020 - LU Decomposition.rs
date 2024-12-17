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

    let n = scan.token::<usize>();
    let mut a = vec![0.0; n];
    let mut b = vec![0.0; n];
    let mut c = vec![0.0; n];

    for i in 0..n {
        for j in 0..n {
            let val = scan.token::<f64>();

            if i == j {
                b[i] = val;
            } else if i > 0 && j == i - 1 {
                a[i] = val;
            } else if i < n - 1 && j == i + 1 {
                c[i] = val;
            }
        }
    }

    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];

    for i in 0..n {
        l[i][i] = 1.0;
    }

    if b[0] == 0.0 {
        writeln!(out, "-1").unwrap();
        return;
    }

    u[0][0] = b[0];

    if n > 1 {
        u[0][1] = c[0];
    }

    for i in 1..n {
        let diag_u = u[i - 1][i - 1];

        if diag_u == 0.0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        l[i][i - 1] = a[i] / diag_u;

        let val = b[i] - l[i][i - 1] * c[i - 1];

        if val == 0.0 {
            writeln!(out, "-1").unwrap();
            return;
        }

        u[i][i] = val;

        if i < n - 1 {
            u[i][i + 1] = c[i];
        }
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{:.3} ", l[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{:.3} ", u[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
