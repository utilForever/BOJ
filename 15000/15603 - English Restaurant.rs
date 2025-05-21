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

#[inline]
fn tri(x: i64) -> f64 {
    (x as f64) * ((x + 1) as f64) * 0.5
}

#[inline]
fn add(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 + b.0, a.1 + b.1)
}

#[inline]
fn mul(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 * b.1 + a.1 * b.0, a.1 * b.1)
}

// Reference: NWERC 2017 Presentation of Solutions
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, g, t) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut capacities = vec![0; n];

    for i in 0..n {
        let c = scan.token::<i64>();
        capacities[i] = c.min(g);
    }

    // 1. Sort tables by capacity.
    capacities.sort_unstable();

    // Precompute binomial coefficients
    let mut binomial = vec![Vec::new(); 201];

    for i in 0..=200 {
        binomial[i] = vec![0.0; i + 1];
        binomial[i][0] = 1.0;
        binomial[i][i] = 1.0;

        for j in 1..i {
            binomial[i][j] = binomial[i - 1][j - 1] + binomial[i - 1][j];
        }
    }

    // 3. Add t virtual tables of capacity g, holding the people leaving restaurant.
    capacities.extend(std::iter::repeat(g + 1).take(t));

    let mut dp = vec![vec![(0.0, 0.0); n + t]; n + t];

    // 2. Calculate exptected occupancy E(i, j) for consecutive intervals of tables between i and j,
    //    conditioned upon the interval being fully occupied and the rest being empty.
    for len in 1..=n + t {
        for i in 0..=n + t - len {
            let j = i + len - 1;

            // 4. Dynamic programming, from smallest intervals to the longest.
            //    Pick the last table k occupied in an interval [i, j].
            //    Intervals [i, k - 1] and [k + 1, j] have been occupied before.
            //    There are (j - i + 1) * (k - i) * (j - k) ways of interleaving these two parts.
            for k in i..=j {
                let ways = binomial[len - 1][k - i];
                let people_add = if capacities[k] == g + 1 {
                    0.0
                } else {
                    tri(capacities[k]) - if i > 0 { tri(capacities[i - 1]) } else { 0.0 }
                };
                let cnt_range = (capacities[k].min(g)
                    - if i > 0 { capacities[i - 1].min(g) } else { 0 })
                    as f64;

                let mut val = (0.0, ways);
                val = mul(val, (people_add, cnt_range));

                if k > i {
                    val = mul(val, dp[i][k - 1]);
                }

                if k < j {
                    val = mul(val, dp[k + 1][j]);
                }

                dp[i][j] = add(dp[i][j], val);
            }
        }
    }

    // 5. Use consecutive occupancies to calculate non-consecutive occupancies:
    //    F(k, l) is the average occupancy of the first k tables when l of those tables are occupied.
    let mut f = vec![vec![(0.0, 0.0); n + t]; t];

    for left in 0..t {
        for pos in 0..=n + left {
            f[left][pos] = dp[pos][pos + t - left - 1];
        }
    }

    // 6. F is calculated similarly to E.
    let mut prefix = vec![vec![(0.0, 0.0); n + t]; t];

    for left in (0..t).rev() {
        for pos in (0..n + t).rev() {
            for k in left + 1..t {
                if pos + k - left + 1 >= n + t {
                    continue;
                }

                let mut val = (0.0, binomial[t - left][t - k]);
                val = mul(val, dp[pos][pos + k - left - 1]);
                val = mul(val, prefix[k][pos + k - left + 1]);

                f[left][pos] = add(f[left][pos], val);
            }

            prefix[left][pos] = f[left][pos];

            if pos + 1 < n + t {
                prefix[left][pos] = add(prefix[left][pos], prefix[left][pos + 1]);
            }
        }
    }

    // 7. Final answer is F(n + t, t) - expected occupancy of the n + t tables when t of them are occupied.
    let mut ret = (0.0, 0.0);

    for i in 0..n + t {
        ret = add(ret, f[0][i]);
    }

    writeln!(out, "{:.12}", ret.0 / ret.1).unwrap();
}
