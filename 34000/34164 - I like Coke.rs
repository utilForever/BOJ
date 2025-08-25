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

const MOD: i64 = 998_244_353;

fn calculate(
    comb: &Vec<Vec<i64>>,
    cnt: &mut i64,
    n: i64,
    m: i64,
    rem: i64,
    height: i64,
    dp: &mut HashMap<(i64, i64, i64), i64>,
) -> i64 {
    if dp.contains_key(&(rem, height, *cnt)) {
        return *dp.get(&(rem, height, *cnt)).unwrap();
    }

    if rem == 0 {
        let mut val = 0;

        if *cnt % 3 == (n + m) % 3 && *cnt >= (n % 3 + m % 3) {
            let min = (*cnt - m).max(n % 3);
            let max = (*cnt - m % 3).min(n);
            let mut offset = 0;

            for i in (min..=max).step_by(3) {
                val = (val
                    + comb[*cnt as usize][i as usize]
                        * comb[(n + m - *cnt) as usize / 3][(n - min) as usize / 3 - offset])
                    % MOD;
                offset += 1;
            }
        }

        dp.insert((rem, height, *cnt), val);

        return val;
    }

    if height == 1 {
        *cnt += rem;

        let mut val = 0;

        if *cnt % 3 == (n + m) % 3 && *cnt >= (n % 3 + m % 3) {
            let min = (*cnt - m).max(n % 3);
            let max = (*cnt - m % 3).min(n);

            for i in (min..=max).step_by(3) {
                val = (val
                    + comb[*cnt as usize][i as usize]
                        * comb[(n + m - *cnt) as usize / 3][(n - i) as usize / 3])
                    % MOD;
            }
        }

        *cnt -= rem;

        dp.insert((rem, height, *cnt), val);

        return val;
    }

    let mut val = 0;

    for i in 1..=height {
        if i * (i + 1) / 2 > rem {
            break;
        } else {
            if i % 3 == 1 {
                *cnt += 1;
            }

            val = (val + calculate(comb, cnt, n, m, rem - i * (i + 1) / 2, i, dp)) % MOD;

            if i % 3 == 1 {
                *cnt -= 1;
            }
        }
    }

    dp.insert((rem, height, *cnt), val);

    return val;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut comb = vec![vec![0; 2001]; 2001];

    for i in 0..=2000 {
        comb[i][0] = 1;
        comb[i][i] = 1;

        for j in 1..i {
            comb[i][j] = (comb[i - 1][j - 1] + comb[i - 1][j]) % MOD;
        }
    }

    let mut dp = HashMap::new();
    let mut cnt = 0;

    writeln!(
        out,
        "{}",
        calculate(&comb, &mut cnt, n, m, n + m, n + m, &mut dp)
    )
    .unwrap();
}
