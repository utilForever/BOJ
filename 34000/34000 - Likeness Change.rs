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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut a = vec![0; n + 1];
    let mut b = vec![0; n + 1];

    for i in 1..=n {
        a[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        let val = scan.token::<i64>();
        b[i] = if val == 1 { 1 } else { -1 };
    }

    let mut cnt = vec![0; n + 1];

    for i in 1..=n {
        cnt[a[i] as usize] += 1;
    }

    let mut total = (1..=n).map(|idx| b[a[idx] as usize]).sum::<i64>();

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (i, j) = (scan.token::<usize>(), scan.token::<i64>());

            if a[i] != j {
                let b_old = b[a[i] as usize];
                let b_new = b[j as usize];

                total += b_new - b_old;
                cnt[a[i] as usize] -= 1;
                cnt[j as usize] += 1;
                a[i] = j;
            }
        } else {
            let i = scan.token::<usize>();
            let diff = -2 * b[i] * cnt[i];

            total += diff;
            b[i] *= -1;
        }

        let a = if total >= 0 {
            total / 2
        } else {
            (total - 1) / 2
        };
        let b = total - a;

        writeln!(out, "{}", a * b).unwrap();
    }
}
