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

fn shift_right(arr: &mut [usize], shift: usize) {
    let len = arr.len();
    let temp = arr.to_vec();

    for i in 0..len {
        arr[(i + shift) % len] = temp[i];
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, c, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut r = vec![0; k];

    for i in 0..k {
        r[i] = scan.token::<usize>();
    }

    r.sort();

    let mut ret = vec![vec![0; n]; c];
    let mut val = 1_000_000;

    for i in 0..k {
        let r_prev = if i == 0 { 0 } else { r[i - 1] };
        let vals = (val - r[i] + r_prev + 1..=val).rev();

        for j in 0..c {
            for (k, v) in vals.clone().enumerate() {
                ret[j][r_prev + k] = v;
            }

            shift_right(&mut ret[j][r_prev..r[i]], j);
        }

        val -= r[i] - r_prev;
    }

    let r_max = if k == 0 { 0 } else { r[k - 1] };
    let mut val = 1;

    for i in 0..c {
        for j in r_max..n {
            ret[i][j] = val;
            val = if val == 1_000_000 { 1 } else { val + 1 };
        }
    }

    for i in 0..c {
        for j in 0..n {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
