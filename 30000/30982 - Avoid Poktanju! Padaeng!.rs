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

    let (n, m, t) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut groups = vec![0; n + 1];

    for i in 1..=n {
        groups[i] = scan.token::<i64>();
    }

    let p = scan.token::<usize>();

    let mut min_time_left = vec![i64::MAX / 4; m + 1];
    let mut min_time_right = vec![i64::MAX / 4; m + 1];

    min_time_left[0] = 0;
    min_time_right[0] = 0;

    // Left
    for time in 1..p {
        let next = p - time;
        let group = groups[next] as usize;

        for remain in (group..=m).rev() {
            if min_time_left[remain - group] == i64::MAX / 4 {
                continue;
            }

            min_time_left[remain] = min_time_left[remain].min(time as i64);
        }
    }

    // Right
    for time in 1..=n - p {
        let next = p + time;
        let group = groups[next] as usize;

        for remain in (group..=m).rev() {
            if min_time_right[remain - group] == i64::MAX / 4 {
                continue;
            }

            min_time_right[remain] = min_time_right[remain].min(time as i64);
        }
    }

    let mut left = 0 as i64;
    let mut right = m as i64 - groups[p];

    // Calculate the sum of the shortest time and the longest time according to left and right
    while right >= 0 {
        let time_short = min_time_left[left as usize].min(min_time_right[right as usize]);
        let time_long = min_time_left[left as usize].max(min_time_right[right as usize]);
        let time_total = time_short * 2 + time_long;

        if time_total <= t {
            writeln!(out, "YES").unwrap();
            return;
        }

        left += 1;
        right -= 1;
    }

    writeln!(out, "NO").unwrap();
}
