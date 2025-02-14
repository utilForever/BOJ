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

fn solve(b: &Vec<i64>, n: usize, m: i64, ch: i64) -> Vec<i64> {
    {
        let mut val_b = 1987654321987654321;

        for i in 2..=n {
            let min = -m - b[i];
            let max = m - b[i];

            let val_a = min;
            val_b = val_b.min(max);

            if (val_b & 1) != ch {
                val_b -= 1;
            }

            if val_a > val_b {
                return Vec::new();
            }
        }
    }

    let mut val_a = -1987654321987654321;
    let mut val_b = 1987654321987654321;

    for i in 2..=n {
        let min = -m - b[i];
        let max = m - b[i];

        val_a = val_a.max(min);

        if (val_a & 1) != ch {
            val_a += 1;
        }

        val_b = val_b.min(max);

        if (val_b & 1) != ch {
            val_b -= 1;
        }
    }

    let mut ret = vec![0];
    let mut c = if val_a >= val_b {
        val_a
    } else if val_a <= 0 && val_b >= 0 {
        0
    } else if val_a.abs() > val_b.abs() {
        val_b
    } else {
        val_a
    };

    for i in 3..=n {
        let max = m - b[i];
        let mut c_new = c.min(max);

        if (c_new & 1) != ch {
            c_new -= 1;
        }

        ret.push((c - c_new) / 2);
        c = c_new;
    }

    ret.push(0);

    let mut val = 0;

    for i in 0..n {
        val += ret[i];
    }

    c = if val_a >= val_b {
        val_a
    } else if val_a <= 0 && val_b >= 0 {
        0
    } else if val_a.abs() > val_b.abs() {
        val_b
    } else {
        val_a
    };

    if val > c {
        ret[0] = val - c;
    } else {
        *ret.last_mut().unwrap() = c - val;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
        b[i] = a[i] - a[i - 1];
    }

    if n == 1 {
        writeln!(out, "0").unwrap();
        writeln!(out, "0").unwrap();
        return;
    }

    let x = solve(&b, n, m, 0);
    let y = solve(&b, n, m, 1);
    let mut val_a = 0;
    let mut val_b = 0;

    for val in x.iter() {
        val_a += val;
    }

    for val in y.iter() {
        val_b += val;
    }

    if x.is_empty() {
        if y.is_empty() {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{val_b}").unwrap();

            for val in y.iter() {
                write!(out, "{val} ").unwrap();
            }

            writeln!(out).unwrap();
        }
    } else {
        if y.is_empty() || val_a < val_b {
            writeln!(out, "{val_a}").unwrap();

            for val in x.iter() {
                write!(out, "{val} ").unwrap();
            }

            writeln!(out).unwrap();
        } else {
            writeln!(out, "{val_b}").unwrap();

            for val in y.iter() {
                write!(out, "{val} ").unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
