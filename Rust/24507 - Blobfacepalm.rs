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

    if n == 4 {
        writeln!(out, "Yes").unwrap();
        writeln!(out, "1 2 1 3 2 0 0 3").unwrap();
        return;
    }

    if n == 5 {
        writeln!(out, "Yes").unwrap();
        writeln!(out, "2 4 1 2 1 3 4 0 0 3").unwrap();
        return;
    }

    match n % 4 {
        0 => {
            let mut arr = vec![0; (n + 1) * 2];
            let k = n as i64 / 4;

            for r in 0..=(2 * k - 1) {
                let val = (8 * k - r) - (4 * k + r);
                arr[(4 * k + r) as usize] = val;
                arr[(8 * k - r) as usize] = val;
            }

            {
                let val = 6 * k - (2 * k + 1);
                arr[(2 * k + 1) as usize] = val;
                arr[(6 * k) as usize] = val;
            }

            {
                let val = (4 * k - 1) - (2 * k);
                arr[(2 * k) as usize] = val;
                arr[(4 * k - 1) as usize] = val;
            }

            for r in 1..=(k - 1) {
                let val = (4 * k - 1 - r) - r;
                arr[r as usize] = val;
                arr[(4 * k - 1 - r) as usize] = val;
            }

            arr[k as usize] = 1;
            arr[(k + 1) as usize] = 1;

            for r in 0..=(k - 3) {
                let val = (3 * k - 1 - r) - (k + 2 + r);
                arr[(k + 2 + r) as usize] = val;
                arr[(3 * k - 1 - r) as usize] = val;
            }

            writeln!(out, "Yes").unwrap();

            for idx in 1..=(2 * n) {
                write!(out, "{} ", arr[idx] - 1).unwrap();
            }

            writeln!(out).unwrap();
        }
        1 => {
            let mut arr = vec![0; (n + 1) * 2];
            let k = n as i64 / 4;

            for r in 0..=(2 * k - 1) {
                let val = (8 * k + 2 - r) - (4 * k + 2 + r);
                arr[(4 * k + 2 + r) as usize] = val;
                arr[(8 * k + 2 - r) as usize] = val;
            }

            {
                let val = (6 * k + 2) - (2 * k + 1);
                arr[(2 * k + 1) as usize] = val;
                arr[(6 * k + 2) as usize] = val;
            }

            {
                let val = (4 * k + 1) - (2 * k + 2);
                arr[(2 * k + 2) as usize] = val;
                arr[(4 * k + 1) as usize] = val;
            }

            for r in 1..=k {
                let val = (4 * k + 1 - r) - r;
                arr[r as usize] = val;
                arr[(4 * k + 1 - r) as usize] = val;
            }

            arr[(k + 1) as usize] = 1;
            arr[(k + 2) as usize] = 1;

            for r in 1..=(k - 2) {
                let val = (3 * k + 1 - r) - (k + 2 + r);
                arr[(k + 2 + r) as usize] = val;
                arr[(3 * k + 1 - r) as usize] = val;
            }

            writeln!(out, "Yes").unwrap();

            for idx in 1..=(2 * n) {
                write!(out, "{} ", arr[idx] - 1).unwrap();
            }

            writeln!(out).unwrap();
        }
        2 | 3 => writeln!(out, "No").unwrap(),
        _ => unreachable!(),
    }
}
