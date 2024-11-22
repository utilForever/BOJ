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

fn multiply(a: Vec<Vec<i64>>, b: Vec<Vec<i64>>, m: i64) -> Vec<Vec<i64>> {
    let mut ret = vec![vec![0, 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            ret[i][j] = 0;

            for k in 0..2 {
                ret[i][j] += a[i][k] * b[k][j];
                ret[i][j] %= m;
            }
        }
    }

    ret
}

fn fibonacci(n: i64, m: i64) -> Vec<Vec<i64>> {
    let multiplier = vec![vec![1, 1], vec![1, 0]];

    if n == 1 {
        return multiplier;
    }

    if n % 2 == 1 {
        let rest = fibonacci(n - 1, m);
        multiply(multiplier, rest, m)
    } else {
        let rest = fibonacci(n / 2, m);
        multiply(rest.clone(), rest, m)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut nums = vec![(0, 0); 2 * k];

    for i in 0..k {
        let (idx, val) = (scan.token::<i64>(), scan.token::<i64>());
        nums[2 * i] = (idx, val);
        nums[2 * i + 1] = (val, val);
    }

    if m == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    if k == 0 {
        writeln!(out, "{}", fibonacci(2 * n, m)[0][1] % m).unwrap();
        return;
    }

    nums.sort();
    nums.dedup();

    let mut ret = fibonacci(2 * nums[0].0 - 1, m)[0][1] % m;

    for i in 1..nums.len() {
        if nums[i - 1].1 > nums[i].1 {
            writeln!(out, "0").unwrap();
            return;
        }

        if nums[i - 1].1 == nums[i].1 {
            continue;
        }

        ret = ret * fibonacci(2 * (nums[i].0 - nums[i - 1].0), m)[0][1] % m;
    }

    writeln!(
        out,
        "{}",
        ret * fibonacci(2 * (n - nums.last().unwrap().0) + 1, m)[0][1] % m
    )
    .unwrap();
}
