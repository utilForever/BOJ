use io::Write;
use std::{cmp, io, str};

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

fn solve_greedy(coins: &Vec<i64>, temp: &mut Vec<i64>, s: i64) -> i64 {
    if temp[s as usize] > -1 {
        return temp[s as usize];
    }

    for i in (0..=coins.len() - 1).rev() {
        if s >= coins[i] {
            temp[s as usize] = solve_greedy(coins, temp, s - coins[i]) + 1;
            break;
        }
    }

    temp[s as usize]
}

fn solve_optimal(coins: &Vec<i64>, temp: &mut Vec<i64>, s: i64) -> i64 {
    if s < 0 {
        return 1_000_000_000;
    }

    if temp[s as usize] > -1 {
        return temp[s as usize];
    }

    let mut min_coins = solve_optimal(coins, temp, s - coins[0]) + 1;
    for i in 1..coins.len() {
        min_coins = cmp::min(min_coins, solve_optimal(coins, temp, s - coins[i]) + 1);
    }

    temp[s as usize] = min_coins;

    temp[s as usize]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut coins = vec![0; n];

    for i in 0..n {
        coins[i] = scan.token::<i64>();
    }

    let mut temp1 = vec![-1; (coins[n - 1] + coins[n - 2]) as usize];
    let mut temp2 = vec![-1; (coins[n - 1] + coins[n - 2]) as usize];

    temp1[0] = 0;
    temp2[0] = 0;

    let mut is_canonical = true;

    for i in 1..temp1.len() as i64 {
        if solve_greedy(&coins, &mut temp2, i) != solve_optimal(&coins, &mut temp1, i) {
            is_canonical = false;
            break;
        }
    }
    writeln!(
        out,
        "{}",
        if is_canonical {
            "canonical"
        } else {
            "non-canonical"
        }
    )
    .unwrap();
}
