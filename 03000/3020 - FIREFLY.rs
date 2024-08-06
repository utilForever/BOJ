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

    let (n, h) = (scan.token::<usize>(), scan.token::<usize>());
    let mut obstacles_lower = vec![0; h + 2];
    let mut obstacles_upper = vec![0; h + 2];

    for _ in 0..n / 2 {
        let height = scan.token::<usize>();
        obstacles_lower[height] += 1;

        let height = scan.token::<usize>();
        obstacles_upper[h - height + 1] += 1;
    }

    for i in 1..=h {
        obstacles_lower[i] += obstacles_lower[i - 1];
    }

    for i in (1..=h).rev() {
        obstacles_upper[i] += obstacles_upper[i + 1];
    }

    let mut obstacles_min = i64::MAX;
    let mut cnt_intervals = 0;

    for i in 1..=h {
        let num_obstacles = obstacles_lower[h] - obstacles_lower[i - 1] + obstacles_upper[1]
            - obstacles_upper[i + 1];

        if num_obstacles < obstacles_min {
            obstacles_min = num_obstacles;
            cnt_intervals = 1;
        } else if num_obstacles == obstacles_min {
            cnt_intervals += 1;
        }
    }

    writeln!(out, "{obstacles_min} {cnt_intervals}").unwrap();
}
