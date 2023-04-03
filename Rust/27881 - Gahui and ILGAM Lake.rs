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
    let mut points = vec![0; 4 * n + 1];

    for i in 1..=4 * n {
        points[i] = scan.token::<i64>();
        points[i] += points[i - 1];
    }

    let mut stations = vec![0; 5];

    for i in 1..=4 {
        let (konkuk_univ, guui, sejong_univ) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        stations[i] = konkuk_univ.min(guui.min(sejong_univ));
    }

    let calculate_dist = |points: &Vec<i64>, mut a: usize, mut b: usize| -> i64 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }

        return (points[b - 1] - points[a - 1]).min(points[4 * n] - points[b - 1] + points[a - 1]);
    };

    let q = scan.token::<i64>();

    for _ in 0..q {
        let k = scan.token::<usize>();
        let mut ret = i64::MAX;

        for i in 1..=4 {
            ret = ret.min(stations[i] + calculate_dist(&points, k, (i * n) as usize));
        }

        writeln!(out, "{ret}").unwrap();
    }
}
