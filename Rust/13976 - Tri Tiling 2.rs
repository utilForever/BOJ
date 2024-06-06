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

const MOD: i64 = 1_000_000_007;

fn matrix_multiply(first: [[i64; 2]; 2], second: [[i64; 2]; 2]) -> [[i64; 2]; 2] {
    let mut ret = [[0; 2]; 2];

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                ret[i][j] = (MOD + ret[i][j] + first[i][k] * second[k][j]) % MOD;
            }
        }
    }

    ret
}

fn matrix_pow(mut matrix: [[i64; 2]; 2], mut n: usize) -> [[i64; 2]; 2] {
    let mut ret = [[1, 0], [0, 1]];

    if n == 0 {
        return ret;
    }

    while n > 0 {
        if n % 2 == 1 {
            ret = matrix_multiply(ret, matrix);
        }

        matrix = matrix_multiply(matrix, matrix);
        n /= 2;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    // Slow solution
    // for i in (2..=n).step_by(2) {
    //     ret[i] = (ret[i - 2] * 3) % MOD;

    //     if i < 4 {
    //         continue;
    //     }

    //     for j in (0..=i - 4).step_by(2) {
    //         ret[i] = (ret[i] + ret[j] * 2) % MOD;
    //     }
    // }

    // Solution
    // Case 1: n is odd
    // No solution
    // Case 2: n is even
    // DP formula : f(n) = 3 * f(n - 2) + sum(f(k)) for k in (0..n - 4) by 2
    //              f(n) = 4 * f(n - 2) - f(n - 4)
    // Matrix form: [f(n), f(n - 2)] = [[4, -1], [1, 0]] * [f(n - 2), f(n - 4)]
    //              [f(n), f(n - 2)] = [[4, -1], [1, 0]] ^ (n / 2) * [f(0), f(-2)]
    //              [f(n), f(n - 2)] = [[4, -1], [1, 0]] ^ (n / 2) * [1, 0]

    if n % 2 == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let base = [[1, 0], [1, 0]];
    let matrix = [[4, -1], [1, 0]];
    let ret = matrix_pow(matrix, n / 2);
    let ret = matrix_multiply(ret, base);

    writeln!(out, "{}", ret[0][0] % MOD).unwrap();
}
