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

static mut POWERS: [i64; 8001] = [0; 8001];
static mut POWERS_ACC: [i64; 8001] = [0; 8001];
static mut RISKS: [[i64; 8001]; 801] = [[0; 8001]; 801];

unsafe fn calculate_min(idx: usize, left: usize, right: usize, p_left: usize, p_right: usize) {
    let mid = (left + right) / 2;
    let mut idx_opt = -1;
    RISKS[idx][mid] = i64::MAX;

    for i in p_left..=p_right.min(mid) {
        let risk =
            RISKS[idx - 1][i - 1] + (mid - i + 1) as i64 * (POWERS_ACC[mid] - POWERS_ACC[i - 1]);

        if risk < RISKS[idx][mid] {
            idx_opt = i as i64;
            RISKS[idx][mid] = risk;
        }
    }

    if left != mid {
        calculate_min(idx, left, mid - 1, p_left, idx_opt as usize);
    }

    if right != mid {
        calculate_min(idx, mid + 1, right, idx_opt as usize, p_right);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (l, g) = (scan.token::<usize>(), scan.token::<usize>());

        unsafe {
            for i in 1..=l {
                POWERS[i] = scan.token::<i64>();
                POWERS_ACC[i] = POWERS_ACC[i - 1] + POWERS[i];
            }

            for i in 1..=l {
                RISKS[1][i] = POWERS_ACC[i] * i as i64;
            }

            for i in 2..=g {
                calculate_min(i, 1, l, 1, l);
            }

            writeln!(out, "{}", RISKS[g][l]).unwrap();
        }
    }
}
