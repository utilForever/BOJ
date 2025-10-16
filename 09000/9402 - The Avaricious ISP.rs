use io::Write;
use std::{
    cmp::Ordering,
    f64::consts::{FRAC_PI_2, PI},
    io, str,
};

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

fn calculate(points: &Vec<(i64, i64, i64)>, angle: f64, sum_values: i64) -> i64 {
    let mut arr: Vec<(f64, i64, usize)> = points
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            (
                (p.1 as f64) * angle.cos() + (p.2 as f64) * angle.sin(),
                p.0,
                idx,
            )
        })
        .collect();

    arr.sort_unstable_by(|a, b| match a.0.partial_cmp(&b.0).unwrap() {
        Ordering::Equal => a.2.cmp(&b.2),
        ord => ord,
    });

    let mut sum = 0;
    let mut ret = 0;

    for i in 0..arr.len().saturating_sub(1) {
        sum += arr[i].1;
        ret = ret.max(sum * (sum_values - sum));
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut points = vec![(0, 0, 0); n];

        for i in 0..n {
            points[i] = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );
        }

        if n == 1 {
            writeln!(out, "0").unwrap();
            continue;
        }

        let sum_values = points.iter().map(|p| p.0).sum();
        let mut angles = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = (points[j].1 - points[i].1) as f64;
                let dy = (points[j].2 - points[i].2) as f64;
                let angle = (dy.atan2(dx) + FRAC_PI_2).rem_euclid(PI);

                angles.push(angle);
            }
        }

        angles.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        angles.dedup();

        let mut ret = 0;

        for i in 0..angles.len() {
            let angle1 = angles[i];
            let angle2 = if i + 1 < angles.len() {
                angles[i + 1]
            } else {
                angles[0] + PI
            };

            let mid = ((angle1 + angle2) / 2.0).rem_euclid(PI);
            let val = calculate(&points, mid, sum_values);

            ret = ret.max(val);
        }

        writeln!(out, "{ret}").unwrap();
    }
}
