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

fn check(
    points: &Vec<(i64, i64)>,
    u_min: i64,
    u_max: i64,
    v_min: i64,
    v_max: i64,
    diameter: i64,
    need: usize,
) -> bool {
    let rectangles = [
        (u_min, u_min + diameter, v_min, v_min + diameter),
        (u_min, u_min + diameter, v_max - diameter, v_max),
        (u_max - diameter, u_max, v_min, v_min + diameter),
        (u_max - diameter, u_max, v_max - diameter, v_max),
    ];

    let mut freq = [0; 16];

    for &(u, v) in points.iter() {
        let mut mask = 0;

        for (idx, &(u_left, u_right, v_left, v_right)) in rectangles.iter().enumerate() {
            if u_left > u || u > u_right || v_left > v || v > v_right {
                continue;
            }

            mask |= 1usize << idx;
        }

        freq[mask] += 1;
    }

    let mut cnt = [0; 4];

    for mask in 0..16 {
        if freq[mask] == 0 {
            continue;
        }

        for i in 0..4 {
            if (mask & (1usize << i)) != 0 {
                cnt[i] += freq[mask];
            }
        }
    }

    for i in 0..4 {
        if cnt[i] < need {
            continue;
        }

        for j in 0..4 {
            if cnt[j] < need {
                continue;
            }

            let mut cnt_covered = 0;

            for mask in 0..16 {
                if (mask & ((1usize << i) | (1usize << j))) != 0 {
                    cnt_covered += freq[mask];
                }
            }

            if cnt_covered == points.len() {
                return true;
            }
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
        let mut points = vec![(0, 0); n];
        let mut u_min = i64::MAX;
        let mut u_max = i64::MIN;
        let mut v_min = i64::MAX;
        let mut v_max = i64::MIN;

        for i in 0..n {
            let (x, y) = (scan.token::<i64>(), scan.token::<i64>());
            let (u, v) = (x + y, x - y);

            points[i] = (u, v);
            u_min = u_min.min(u);
            u_max = u_max.max(u);
            v_min = v_min.min(v);
            v_max = v_max.max(v);
        }

        let mut left = 0;
        let mut right = (u_max - u_min).max(v_max - v_min);

        while left < right {
            let mid = (left + right) / 2;

            if check(
                &points,
                u_min,
                u_max,
                v_min,
                v_max,
                mid,
                n.saturating_sub(k),
            ) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        writeln!(out, "{}", left / 2).unwrap();
    }
}
