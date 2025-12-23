use io::Write;
use std::{collections::HashMap, io, str};

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

fn factor_exponents(primes: &Vec<i64>, mut n: i64) -> Vec<i64> {
    let mut exponents = Vec::new();

    for &prime in primes.iter() {
        if prime * prime > n {
            break;
        }

        if n % prime == 0 {
            let mut cnt = 0;

            while n % prime == 0 {
                n /= prime;
                cnt += 1;
            }

            exponents.push(cnt);
        }
    }

    if n > 1 {
        exponents.push(1);
    }

    exponents
}

fn process_dfs(
    exponents: &mut Vec<i64>,
    dp: &mut HashMap<(i64, Vec<i64>), bool>,
    last: i64,
) -> bool {
    let key = (last, exponents.to_vec());

    if let Some(&ret) = dp.get(&key) {
        return ret;
    }

    let len = exponents.len();
    let mut can_win = false;

    for i in 0..len {
        if exponents[i] > 0 && i as i64 != last {
            exponents[i] -= 1;
            let opponent_wins = process_dfs(exponents, dp, i as i64);
            exponents[i] += 1;

            if !opponent_wins {
                can_win = true;
                break;
            }
        }
    }

    dp.insert(key, can_win);
    can_win
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut is_prime = vec![true; 1_000_001];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=(1_000_000 as f64).sqrt() as usize {
        if is_prime[i] {
            for j in (i * i..=1_000_000).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    let mut primes = Vec::new();

    for i in 2..=1_000_000 {
        if is_prime[i] {
            primes.push(i as i64);
        }
    }

    let t = scan.token::<i64>();
    let mut cache = HashMap::new();

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut exponents = factor_exponents(&primes, n);
        let mut dp = HashMap::new();
        let ret = if cache.contains_key(&exponents) {
            *cache.get(&exponents).unwrap()
        } else {
            let val = process_dfs(&mut exponents, &mut dp, 255);
            cache.insert(exponents.clone(), val);
            val
        };

        writeln!(out, "{}", if ret { "yyyy7089" } else { "toycartoon" }).unwrap();
    }
}
