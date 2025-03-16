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

    let (n, m, q) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut room = vec![0; n + 1];

    for _ in 0..m {
        let (w, d) = (scan.token::<usize>(), scan.token::<i64>());
        room[w] = d;
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + room[i];
    }

    let mut bound_left = 1;
    let mut bound_right = n;

    for _ in 0..q {
        let p = scan.token::<usize>();

        if p < bound_left || p > bound_right {
            writeln!(out, "0").unwrap();
            continue;
        }

        let left = prefix_sum[p] - prefix_sum[bound_left - 1];
        let right = prefix_sum[bound_right] - prefix_sum[p - 1];
        let dist_left = p - 1;
        let dist_right = n - p;

        if left < right || (left == right && dist_left < dist_right) {
            writeln!(out, "{left}").unwrap();
            bound_left = p;
        } else if left > right || (left == right && dist_left > dist_right) {
            writeln!(out, "{right}").unwrap();
            bound_right = p;
        } else {
            writeln!(out, "{left}").unwrap();
            bound_left = p;
        }
    }
}
