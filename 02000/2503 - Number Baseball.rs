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

    let n = scan.token::<i64>();
    let mut candidates = vec![true; 1000];

    for _ in 0..n {
        let (num, strike, ball) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        for i in 123..=987 {
            if !candidates[i] {
                continue;
            }

            let candidate_a = i as i64 / 100;
            let candidate_b = (i as i64 % 100) / 10;
            let candidate_c = (i as i64 % 100) % 10;

            let num_a = num / 100;
            let num_b = (num % 100) / 10;
            let num_c = (num % 100) % 10;

            if candidate_b == 0 || candidate_c == 0 {
                candidates[i] = false;
                continue;
            }

            if candidate_a == candidate_b
                || candidate_a == candidate_c
                || candidate_b == candidate_c
            {
                candidates[i] = false;
                continue;
            }

            let mut cnt_strike = 0;
            let mut cnt_ball = 0;

            if candidate_a == num_a {
                cnt_strike += 1;
            }

            if candidate_b == num_b {
                cnt_strike += 1;
            }

            if candidate_c == num_c {
                cnt_strike += 1;
            }

            if num_a == candidate_b || num_a == candidate_c {
                cnt_ball += 1;
            }

            if num_b == candidate_a || num_b == candidate_c {
                cnt_ball += 1;
            }

            if num_c == candidate_a || num_c == candidate_b {
                cnt_ball += 1;
            }

            if !(cnt_strike == strike && cnt_ball == ball) {
                candidates[i] = false;
            }
        }
    }

    writeln!(
        out,
        "{}",
        candidates[123..=987].iter().filter(|&&x| x).count()
    )
    .unwrap();
}
