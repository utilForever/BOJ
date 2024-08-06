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

fn cos_asin(x: f64) -> f64 {
    (1.0 - x * x).sqrt()
}

fn tan_asin(x: f64) -> f64 {
    x / (1.0 - x * x).sqrt()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (w, _) = (scan.token::<i64>(), scan.token::<i64>());
    let (n, d) = (scan.token::<usize>(), scan.token::<f64>());
    let mut lanes = vec![0.0; n + 1];
    let mut densities = vec![0.0; n];
    let mut idx_d = 0;

    for i in 1..=n {
        lanes[i] = scan.token::<f64>();

        if lanes[i] < d {
            idx_d += 1;
        }
    }

    for i in 0..n {
        densities[i] = scan.token::<f64>();
    }

    let mut idx1 = n as i64 - 1;
    let mut ret = f64::MAX;

    while idx1 >= 0 && lanes[idx1 as usize] >= d {
        let mut val = 0.0;
        let mut idx_left = w as f64;
        let mut idx2 = idx1 - 1;

        while idx2 > idx_d {
            val += densities[idx2 as usize] * (lanes[idx2 as usize + 1] - lanes[idx2 as usize])
                / cos_asin(densities[idx1 as usize] / densities[idx2 as usize])
                * 2.0;
            idx_left -= (lanes[idx2 as usize + 1] - lanes[idx2 as usize])
                * tan_asin(densities[idx1 as usize] / densities[idx2 as usize])
                * 2.0;
            idx2 -= 1;
        }

        while idx2 >= 0 {
            val += densities[idx2 as usize] * (lanes[idx2 as usize + 1] - lanes[idx2 as usize])
                / cos_asin(densities[idx1 as usize] / densities[idx2 as usize]);
            idx_left -= (lanes[idx2 as usize + 1] - lanes[idx2 as usize])
                * tan_asin(densities[idx1 as usize] / densities[idx2 as usize]);
            idx2 -= 1;
        }

        val += densities[idx_d as usize] * (lanes[idx_d as usize + 1] - d)
            / cos_asin(densities[idx1 as usize] / densities[idx_d as usize]);
        idx_left -= (lanes[idx_d as usize + 1] - d)
            * tan_asin(densities[idx1 as usize] / densities[idx_d as usize]);

        if idx_left < 0.0 {
            idx1 -= 1;
            continue;
        }

        ret = ret.min(idx_left * densities[idx1 as usize] + val);
        idx1 -= 1;
    }

    let mut left = 0.0;
    let mut right = 1.0;

    while right - left > f64::EPSILON {
        let mid = (left + right) / 2.0;
        let mut val = 0.0;
        let mut idx_left = w as f64;

        for i in 0..idx_d {
            val += densities[i as usize] * (lanes[i as usize + 1] - lanes[i as usize])
                / cos_asin(mid * densities[0] / densities[i as usize]);
            idx_left -= (lanes[i as usize + 1] - lanes[i as usize])
                * tan_asin(mid * densities[0] / densities[i as usize]);
        }

        val += densities[idx_d as usize] * (d - lanes[idx_d as usize])
            / cos_asin(mid * densities[0] / densities[idx_d as usize]);
        idx_left -=
            (d - lanes[idx_d as usize]) * tan_asin(mid * densities[0] / densities[idx_d as usize]);

        if idx_left > 0.0 {
            left = mid;
        } else {
            ret = ret.min(val);
            right = mid;
        }
    }

    if d == 0.0 {
        ret = ret.min(w as f64 * densities[0]);
    }

    writeln!(out, "{:.10}", ret).unwrap();
}
