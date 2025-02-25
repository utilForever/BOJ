use io::Write;
use std::{collections::BTreeMap, io, str};

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

pub struct Data {
    pub arr: Vec<u64>,
    pub query: Vec<[usize; 4]>,
}

pub fn generate(n: usize, q: usize, mut seed: u64) -> Data {
    let mut splitmix64 = || {
        seed = seed.wrapping_add(0x9e3779b97f4a7c15_u64);
        seed = (seed ^ (seed >> 30)).wrapping_mul(0xbf58476d1ce4e5b9_u64);
        seed = (seed ^ (seed >> 27)).wrapping_mul(0x94d049bb133111eb_u64);
        seed = seed ^ (seed >> 31);
        seed
    };

    let mut arr = vec![0; n + 1];
    let mut query = vec![[0; 4]; q + 1];

    for v in arr.iter_mut().skip(1) {
        *v = (splitmix64() % 1_000_000) as u64 + 1;
    }

    for v in query.iter_mut().skip(1) {
        let t = (splitmix64() % 4) as usize + 1;
        v[0] = t;

        let mut l = (splitmix64() % n as u64) as usize + 1;
        let mut r = (splitmix64() % n as u64) as usize + 1;

        if l > r {
            std::mem::swap(&mut l, &mut r);
        }

        v[1] = l;
        v[2] = r;

        let m = if t <= 2 { 1_000_000 } else { 998_244_353 };
        v[3] = (splitmix64() % m as u64) as usize + 1;
    }

    Data { arr, query }
}

const MOD_VAL: u64 = 1_000_000;
const MOD: u64 = 998_244_353;

fn pow(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut ret = 1;

    base %= m;

    while exp > 0 {
        if exp & 1 == 1 {
            ret = ret * base % m;
        }

        base = base * base % m;
        exp >>= 1;
    }

    ret
}

fn split_interval(map: &mut BTreeMap<usize, (usize, u64)>, pos: usize) {
    if pos <= 1 {
        return;
    }

    if let Some((&start, &(end, val))) = map.range(..pos).next_back() {
        if start < pos && pos <= end {
            map.remove(&start);
            map.insert(start, (pos - 1, val));
            map.insert(pos, (end, val));
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q, seed) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<u64>(),
    );
    let data = generate(n, q, seed);

    let catalan_max = n.max(1_000_000);
    let catalan_len = 2 * catalan_max;

    let mut factorial = vec![0; catalan_len + 1];
    factorial[0] = 1;

    for i in 1..=catalan_len {
        factorial[i] = factorial[i - 1] * i as u64 % MOD;
    }

    let mut factorial_inv = vec![0; catalan_len + 1];
    factorial_inv[catalan_len] = pow(factorial[catalan_len], MOD - 2, MOD);

    for i in (0..catalan_len).rev() {
        factorial_inv[i] = factorial_inv[i + 1] * (i + 1) as u64 % MOD;
    }

    let mut catalan = vec![0; catalan_len + 1];
    catalan[0] = 1;

    for i in 1..=catalan_max {
        catalan[i] = factorial[2 * i] * factorial_inv[i] % MOD * factorial_inv[i + 1] % MOD;
    }

    let mut catalan_prefix_sum = vec![0; n + 1];

    for i in 1..=n {
        catalan_prefix_sum[i] = (catalan_prefix_sum[i - 1] + catalan[i]) % MOD;
    }

    let mut tree: BTreeMap<usize, (usize, u64)> = BTreeMap::new();
    let mut i = 1;

    while i <= n {
        let mut j = i;

        while j <= n && data.arr[i] == data.arr[j] {
            j += 1;
        }

        tree.insert(i, (j - 1, data.arr[i]));
        i = j;
    }

    for i in 1..=q {
        let (command, l, r, x) = (
            data.query[i][0],
            data.query[i][1],
            data.query[i][2],
            data.query[i][3] as u64,
        );

        split_interval(&mut tree, l);
        split_interval(&mut tree, r + 1);

        if command == 1 {
            let keys = tree.range(l..=r).map(|(&k, _)| k).collect::<Vec<usize>>();

            for key in keys {
                tree.remove(&key);
            }

            let mut left_new = l;
            let mut right_new = r;

            if let Some((&prev_start, &(prev_end, prev_val))) = tree.range(..l).next_back() {
                if prev_end + 1 == l && prev_val == x {
                    left_new = prev_start;
                    tree.remove(&prev_start);
                }
            }

            if let Some((&next_start, &(next_end, next_val))) = tree.range(r + 1..).next() {
                if next_start == r + 1 && next_val == x {
                    right_new = next_end;
                    tree.remove(&next_start);
                }
            }

            tree.insert(left_new, (right_new, x));
        } else if command == 2 {
            let keys = tree.range(l..=r).map(|(&k, _)| k).collect::<Vec<usize>>();

            for key in keys.iter() {
                if let Some((_, val)) = tree.get_mut(&key) {
                    *val = (*val + x) % MOD_VAL;
                }
            }

            let mut intervals = Vec::new();
            let keys = tree.range(l..=r).map(|(&k, _)| k).collect::<Vec<usize>>();

            for key in keys {
                if let Some(&(end, val)) = tree.get(&key) {
                    intervals.push((key, end, val));
                }

                tree.remove(&key);
            }

            intervals.sort_by_key(|x| x.0);

            let mut merged: Vec<(usize, usize, u64)> = Vec::new();

            for (s, e, v) in intervals {
                if let Some(last) = merged.last_mut() {
                    if last.1 + 1 == s && last.2 == v {
                        last.1 = e;
                    } else {
                        merged.push((s, e, v));
                    }
                } else {
                    merged.push((s, e, v));
                }
            }

            for (s, e, v) in merged.iter() {
                tree.insert(*s, (*e, *v));
            }

            if let Some((&prev_start, &(prev_end, prev_val))) = tree.range(..l).next_back() {
                if prev_end + 1 == l {
                    if let Some(&(curr_end, curr_val)) = tree.get(&l) {
                        if prev_val == curr_val {
                            tree.remove(&l);
                            tree.insert(prev_start, (curr_end, prev_val));
                        }
                    }
                }
            }

            if let Some((&next_start, &(next_end, next_val))) = tree.range(r + 1..).next() {
                if let Some((&curr_start, &(curr_end, curr_val))) = tree.range(..=r).next_back() {
                    if curr_end + 1 == next_start && curr_val == next_val {
                        tree.remove(&next_start);
                        tree.insert(curr_start, (next_end, curr_val));
                    }
                }
            }
        } else if command == 3 {
            let mut ret = 0;

            for (&start, &(end, val)) in tree.range(l..=r) {
                let len = (end - start + 1) as u64;
                let term = pow(catalan[val as usize], x, MOD);

                ret = (ret + len % MOD * term % MOD) % MOD;
            }

            writeln!(out, "{ret}").unwrap();
        } else {
            let mut freq_map = BTreeMap::<u64, usize>::new();

            for (&start, &(end, val)) in tree.range(l..=r) {
                let count = end - start + 1;
                *freq_map.entry(val).or_insert(0) += count;
            }

            let mut offset = 0;
            let mut ret = 0;

            for (val, count) in freq_map {
                let power = pow(val, x, MOD);
                let catalan_sum =
                    (catalan_prefix_sum[offset + count] + MOD - catalan_prefix_sum[offset]) % MOD;

                ret = (ret + power * catalan_sum) % MOD;
                offset += count;
            }

            writeln!(out, "{ret}").unwrap();
        }
    }
}
