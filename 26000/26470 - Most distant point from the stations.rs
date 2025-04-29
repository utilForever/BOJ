use io::Write;
use std::{
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hasher},
    iter::repeat_with,
};
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

#[derive(Clone, Debug)]
pub struct Xorshift {
    y: u64,
}

impl Xorshift {
    pub fn new_with_seed(seed: u64) -> Self {
        Xorshift { y: seed }
    }

    pub fn new() -> Self {
        Xorshift::new_with_seed(RandomState::new().build_hasher().finish())
    }

    pub fn rand64(&mut self) -> u64 {
        self.y ^= self.y << 5;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 11;
        self.y
    }

    pub fn rand(&mut self, k: u64) -> u64 {
        self.rand64() % k
    }

    pub fn rands(&mut self, k: u64, n: usize) -> Vec<u64> {
        repeat_with(|| self.rand(k)).take(n).collect()
    }

    pub fn randf(&mut self) -> f64 {
        const UPPER_MASK: u64 = 0x3FF0_0000_0000_0000;
        const LOWER_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
        let x = self.rand64();
        let tmp = UPPER_MASK | (x & LOWER_MASK);
        let result: f64 = f64::from_bits(tmp);
        f64::from_bits(f64::to_bits(result - 1.0) ^ (x >> 63))
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.randf() < p
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        let mut n = slice.len();
        while n > 1 {
            let i = self.rand(n as _) as usize;
            n -= 1;
            slice.swap(i, n);
        }
    }
}

impl Default for Xorshift {
    fn default() -> Self {
        Xorshift::new_with_seed(0x2b99_2ddf_a232_49d6)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, w, h) = (
        scan.token::<usize>(),
        scan.token::<f64>(),
        scan.token::<f64>(),
    );
    let mut points = vec![(0.0, 0.0); n];

    for i in 0..n {
        points[i] = (scan.token::<f64>(), scan.token::<f64>());
    }

    let mut rng = Xorshift::new();
    let mut ret = 0.0f64;

    for _ in 0..35 {
        let mut x = rng.randf() * w;
        let mut y: f64 = rng.randf() * h;
        let mut dist_min = f64::MAX;

        for i in 0..n {
            let dx = points[i].0 - x;
            let dy = points[i].1 - y;
            dist_min = dist_min.min(dx * dx + dy * dy);
        }

        let mut length_max = w.max(h);

        while length_max > 1e-6 {
            for _ in 0..35 {
                let x_next = x + (rng.randf() * 2.0 - 1.0) * length_max;
                let y_next = y + (rng.randf() * 2.0 - 1.0) * length_max;

                let x_next = x_next.clamp(0.0, w);
                let y_next = y_next.clamp(0.0, h);

                let mut tmp = f64::MAX;

                for i in 0..n {
                    let dx = points[i].0 - x_next;
                    let dy = points[i].1 - y_next;
                    tmp = tmp.min(dx * dx + dy * dy);
                }

                if tmp > dist_min {
                    dist_min = tmp;
                    x = x_next;
                    y = y_next;
                }
            }

            length_max *= 0.9;
        }

        ret = ret.max(dist_min.sqrt());
    }

    writeln!(out, "{:.10}", ret).unwrap();
}
