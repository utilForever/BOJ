use io::Write;
use std::{io, str};

struct Rng([u64; 4]);

impl Rng {
    fn split_mix(v: u64) -> u64 {
        let mut z = v.wrapping_add(0x9e3779b97f4a7c15);

        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    fn new() -> Self {
        let mut seed = 0;
        unsafe { std::arch::x86_64::_rdrand64_step(&mut seed) };

        let mut prev = seed;

        Self(std::array::from_fn(|_| {
            prev = Self::split_mix(prev);
            prev
        }))
    }

    fn next(&mut self, n: u64) -> u64 {
        let [x, y, z, c] = &mut self.0;
        let t = x.wrapping_shl(58) + *c;

        *c = *x >> 6;
        *x = x.wrapping_add(t);

        if *x < t {
            *c += 1;
        }

        *z = z.wrapping_mul(6906969069).wrapping_add(1234567);
        *y ^= y.wrapping_shl(13);
        *y ^= *y >> 17;
        *y ^= y.wrapping_shl(43);

        let base = x.wrapping_add(*y).wrapping_add(*z);
        ((base as u128 * n as u128) >> 64) as u64
    }

    fn range(&mut self, l: u64, r: u64) -> u64 {
        l + self.next(r - l + 1)
    }
}

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

struct Node<'a> {
    n: usize,
    input: &'a [[i64; 4]],
    selected: i128,
    acc: [i64; 4],
}

impl<'a> Node<'a> {
    fn new(input: &'a [[i64; 4]]) -> Self {
        Self {
            n: input.len(),
            input,
            selected: 0,
            acc: [0; 4],
        }
    }

    fn mutate(&mut self, rng: &mut Rng) {
        let idx = rng.range(0, self.n as u64 - 1) as i64;

        self.selected ^= 1 << idx;

        let is_set = (self.selected >> idx) & 1 == 1;

        for i in 0..4 {
            if is_set {
                self.acc[i] += self.input[idx as usize][i];
            } else {
                self.acc[i] -= self.input[idx as usize][i];
            }
        }
    }

    fn score(&self) -> i64 {
        let mut ret = 0;

        for &val in self.acc.iter() {
            ret += val * val;
        }

        -ret
    }

    fn get(&self) -> Vec<usize> {
        let mut ret = Vec::new();

        for i in 0..self.n {
            if (self.selected >> i) & 1 == 1 {
                ret.push(i + 1);
            }
        }

        ret
    }
}

impl<'a> Clone for Node<'a> {
    fn clone(&self) -> Self {
        Self {
            n: self.n,
            input: self.input,
            selected: self.selected,
            acc: self.acc,
        }
    }
}

fn dlas<'a>(rng: &mut Rng, state: &Node<'a>, iter: usize) -> (Node<'a>, i64) {
    let mut states = vec![state.clone(), state.clone(), state.clone()];
    let mut buckets = vec![states[0].score(); 5];

    let mut score_curr = buckets[0];
    let mut score_min = score_curr;
    let mut pos_curr = 0;
    let mut pos_min = 0;

    let mut i = 0;
    let mut k = 0;

    while i < iter {
        let score_prev = score_curr;
        let mut pos_next = if pos_curr + 1 < 3 { pos_curr + 1 } else { 0 };

        if pos_next == pos_min {
            pos_next = if pos_next + 1 < 3 { pos_next + 1 } else { 0 };
        }

        states[pos_next] = states[pos_curr].clone();
        states[pos_next].mutate(rng);

        let score_next = states[pos_next].score();

        if score_min > score_next {
            pos_min = pos_next;
            score_min = score_next;
            i = 0;
        }

        if score_next == score_curr || score_next < *buckets.iter().max().unwrap() {
            pos_curr = pos_next;
            score_curr = score_next;
        }

        if score_curr > buckets[k] || score_curr < buckets[k].min(score_prev) {
            buckets[k] = score_curr;
        }

        i += 1;
        k = if k + 1 < 5 { k + 1 } else { 0 };
    }

    (states[pos_min].clone(), score_min)
}

fn solve<'a>(rng: &mut Rng, input: &'a [[i64; 4]]) -> Vec<usize> {
    let mut optimal = -1;
    let mut ret = Vec::new();

    for _ in 0..128 {
        let (node_state, val) = dlas(rng, &Node::new(input), 100_000);

        if optimal > -val {
            continue;
        }

        ret = node_state.get();
        optimal = -val;
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut powers = vec![[0; 4]; n];

    for i in 0..n {
        powers[i] = [
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        ];
    }

    let mut rng = Rng::new();
    let ret = solve(&mut rng, &powers);

    writeln!(out, "{}", ret.len()).unwrap();

    for val in ret {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
