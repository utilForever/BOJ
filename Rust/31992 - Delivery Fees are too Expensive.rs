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

const MAX: i64 = 10i64.pow(9);

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut delivery_charges = vec![0; m];
    let mut student_charges = vec![vec![0; m]; n];

    for i in 0..m {
        delivery_charges[i] = scan.token::<i64>();
    }

    for i in 0..n {
        for j in 0..m {
            student_charges[i][j] = scan.token::<i64>();
        }
    }

    let mut cost_min = vec![MAX; 1 << n];

    for idx in 0..1 << n {
        let num_students = (idx as i64).count_ones() as i64;

        for i in 0..m {
            let mut charges_min = MAX;

            for j in 0..n {
                if idx & (1 << j) != 0 {
                    charges_min = charges_min.min(student_charges[j][i]);
                }
            }

            if charges_min * num_students >= delivery_charges[i] {
                cost_min[idx] = cost_min[idx].min(delivery_charges[i]);
            }
        }
    }

    let mut dp = vec![MAX; 1 << n];
    dp[0] = 0;

    for idx in 0..1 << n {
        for sub in 0..idx {
            if idx & sub != 0 {
                continue;
            }

            dp[idx | sub] = dp[idx | sub].min(dp[sub] + cost_min[idx]);
        }
    }

    writeln!(
        out,
        "{}",
        if dp[(1 << n) - 1] == MAX {
            -1
        } else {
            dp[(1 << n) - 1]
        }
    )
    .unwrap();
}
