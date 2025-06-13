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

const MOD: i64 = 998_244_353;

fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % MOD
        }

        piv = piv * piv % MOD;
        y >>= 1;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n];

    for _ in 0..n - 1 {
        let (u, v) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut inv = vec![0; n + 1];
    inv[1] = 1;

    for i in 2..=n {
        inv[i] = (MOD - MOD / i as i64) * inv[MOD as usize % i] % MOD;
    }

    // Process DFS for post order
    let mut parent = vec![0; n];
    let mut order = Vec::with_capacity(n);
    let mut stack = vec![0];

    while let Some(node) = stack.pop() {
        order.push(node);

        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            parent[next] = node;
            stack.push(next);
        }
    }

    let mut size = vec![1; n];
    let mut down = vec![0; n];
    let mut pair = vec![0; n];

    for &node in order.iter().rev() {
        let mut prefix_sum = 0;
        let mut sum_size = 0;
        let mut sum_dist = 0;
        let mut sum_pairs = 0;
        let mut cross = 0;

        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            prefix_sum += pair[next];
            cross += (size[next] * sum_dist) + (size[next] + down[next]) * sum_size;
            sum_size += size[next];
            sum_dist += size[next] + down[next];
            sum_pairs += size[next] + down[next];
        }

        prefix_sum += sum_pairs + cross;
        size[node] += sum_size;
        down[node] = sum_pairs;
        pair[node] = prefix_sum;
    }

    // Process DFS for rerooting
    let mut total = vec![0; n];
    let mut stack = vec![0];

    total[0] = down[0];

    while let Some(node) = stack.pop() {
        for &next in graph[node].iter() {
            if next == parent[node] {
                continue;
            }

            total[next] = total[node] + (n as i64 - 2 * size[next]);
            stack.push(next);
        }
    }

    let sum_dist_total = pair[0] % MOD;
    let mut sum_delta = 0;

    for i in 1..n {
        let a = size[i];
        let b = n as i64 - a;

        let dist_a = down[i];
        let dist_b = total[parent[i]] - (a + down[i]);

        let sum_aa = pair[i] % MOD;
        let sum_bb = (pair[0] - pair[i] - (b * dist_a + a * dist_b + a * b)).rem_euclid(MOD);

        let term1 = (2 * b % MOD) * inv[a as usize] % MOD * sum_aa % MOD;
        let term2 = (2 * a % MOD) * inv[b as usize] % MOD * sum_bb % MOD;
        let term3 = b * dist_a % MOD;
        let term4 = a * dist_b % MOD;

        let delta = (term1 + term2 - term3 - term4).rem_euclid(MOD);
        let ab = a * b % MOD;

        sum_delta = (sum_delta + ab * delta) % MOD;
    }

    let m = (n * (n - 1) / 2 - (n - 2)) as i64;
    let numerator = (sum_dist_total * sum_dist_total % MOD + sum_delta) % MOD;
    let mut denomiator = (n as i64 - 1) * m % MOD;
    denomiator = (denomiator * denomiator) % MOD;

    writeln!(out, "{}", numerator * pow(denomiator, MOD - 2) % MOD).unwrap();
}
