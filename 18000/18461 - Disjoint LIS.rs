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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const MOD: i32 = 998_244_353;

fn mod_mul(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % (MOD as i64)) as i32
}

fn mod_exp(mut base: i32, mut exp: i32) -> i32 {
    let mut ret = 1;
    base %= MOD;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = mod_mul(ret, base);
        }

        base = mod_mul(base, base);
        exp >>= 1;
    }

    ret
}

fn mod_inv(a: i32) -> i32 {
    mod_exp(a, MOD - 2)
}

fn dfs_partitions(
    shape: &mut Vec<usize>,
    sum_squares: &mut i32,
    factorial: &[i32],
    factorial_inv: &[i32],
    n: usize,
    max_part: usize,
) {
    if n == 0 {
        let hook_length_product = hook_length_product(shape);
        let hook_length_product_inv = mod_inv(hook_length_product);

        let f_lambda = mod_mul(
            factorial[shape.iter().sum::<usize>()],
            hook_length_product_inv,
        );
        let f_square = mod_mul(f_lambda, f_lambda);

        *sum_squares = (*sum_squares + f_square) % MOD;
        return;
    }

    if max_part == 0 {
        return;
    }

    if max_part <= n {
        shape.push(max_part);
        dfs_partitions(
            shape,
            sum_squares,
            factorial,
            factorial_inv,
            n - max_part,
            max_part,
        );
        shape.pop();
    }

    dfs_partitions(
        shape,
        sum_squares,
        factorial,
        factorial_inv,
        n,
        max_part - 1,
    );
}

pub fn hook_length_product(shape: &[usize]) -> i32 {
    let left = shape.len();
    let mut prod = 1;

    for i in 0..left {
        let len_row = shape[i];

        for j in 0..len_row {
            let right = len_row - j;
            let down = shape[(i + 1)..].iter().filter(|&&r| r > j).count();
            let len_hook = (right + down) as i32;

            prod = mod_mul(prod, len_hook);
        }
    }

    prod
}

// Reference: https://codeforces.com/blog/entry/98167
// Reference: https://youngyojun.github.io/secmem/2021/09/19/young-tableaux/
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();

    if n < 2 {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut factorial = vec![1; n + 1];
    let mut factorial_inv = vec![1; n + 1];

    for i in 1..=n {
        factorial[i] = mod_mul(factorial[i - 1], i as i32);
    }

    factorial_inv[n] = mod_inv(factorial[n]);

    for i in (1..n).rev() {
        factorial_inv[i] = mod_mul(factorial_inv[i + 1], (i + 1) as i32);
    }

    let mut ret = 0;

    for k in 1..=n / 2 {
        let mut shape = Vec::new();
        let r = n - 2 * k;

        shape.push(k);
        shape.push(k);

        let mut sum_squares = 0;

        dfs_partitions(
            &mut shape,
            &mut sum_squares,
            &factorial,
            &factorial_inv,
            r,
            k,
        );

        ret = (ret + sum_squares) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
