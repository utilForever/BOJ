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

fn process_greedy(coins: &Vec<i128>, mut change: i128) -> Vec<i128> {
    let mut ret = vec![0; coins.len()];

    for i in 0..coins.len() {
        if coins[i] <= change {
            let count = change / coins[i];

            ret[i] = count;
            change -= count * coins[i];
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut coins = vec![0; n];

    for i in (0..n).rev() {
        coins[i] = scan.token::<i128>();
    }

    let mut sum_min = i128::MAX;
    let mut flag = true;

    for i in 1..n {
        let base = process_greedy(&coins, coins[i - 1] - 1);

        for j in i..n {
            let mut m = base.clone();
            m[j] += 1;

            for k in j + 1..n {
                m[k] = 0;
            }

            let sum = coins.iter().zip(m.iter()).map(|(a, b)| a * b).sum::<i128>();

            if sum >= sum_min {
                continue;
            }

            let g_sum = process_greedy(&coins, sum).iter().sum::<i128>();
            let m_sum = m.iter().sum::<i128>();

            if g_sum > m_sum {
                flag = false;
                sum_min = sum;
            }
        }
    }

    writeln!(out, "{}", if flag { "Yes" } else { "No" }).unwrap();
}
