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

fn calculate_dist_total(telephone_poles: &Vec<i64>, interval: i64) -> i64 {
    telephone_poles
        .iter()
        .enumerate()
        .skip(1)
        .map(|(idx, val)| (val - idx as i64 * interval).abs())
        .sum::<i64>()
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut telephone_poles = vec![0; n];

    for i in 0..n {
        telephone_poles[i] = scan.token::<i64>();
    }

    let mut left = 0;
    let mut right = telephone_poles[n - 1];

    loop {
        let p1 = (left * 2 + right) / 3;
        let p2 = (left + right * 2) / 3;

        let p1_dist = calculate_dist_total(&telephone_poles, p1);
        let p2_dist = calculate_dist_total(&telephone_poles, p2);

        if p1_dist < p2_dist {
            right = p2;
        } else {
            left = p1;
        }

        if left + 3 > right {
            break;
        }
    }

    let mut ret = i64::MAX;

    for i in left..=right {
        ret = ret.min(calculate_dist_total(&telephone_poles, i));
    }

    writeln!(out, "{ret}").unwrap();
}
