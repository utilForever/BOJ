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

#[derive(Debug, Clone)]
struct Block {
    left: usize,
    right: usize,
    cargo_base: i64,
    cargo_diff: i64,
    strength_base: i64,
    strength_threshold: Vec<i64>,
    prefix_sum_threshold: Vec<i64>,
    idx: usize,
    x_last: i64,
}

impl Block {
    fn new(left: usize, right: usize) -> Self {
        let capacity = right - left + 1;

        Self {
            left,
            right,
            cargo_base: 0,
            cargo_diff: 0,
            strength_base: 0,
            strength_threshold: Vec::with_capacity(capacity),
            prefix_sum_threshold: Vec::with_capacity(capacity + 1),
            idx: 0,
            x_last: i64::MAX,
        }
    }

    fn lower_bound(arr: &Vec<i64>, x: i64) -> usize {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = (left + right) / 2;

            if arr[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    fn rebuild(&mut self, cargos: &Vec<i64>, strengths: &Vec<i64>) {
        let mut cargo_base = 0;
        let mut cargo_diff = 0;

        self.strength_base = 0;
        self.strength_threshold.clear();

        for i in (self.left..=self.right).rev() {
            cargo_diff += cargos[i];

            cargo_base = if cargos[i] > 0 {
                (cargo_base - cargos[i]).max(0)
            } else {
                cargo_base - cargos[i]
            };

            if i == 0 {
                continue;
            }

            let diff = (cargo_base - strengths[i - 1]).max(0);
            let threshold = cargo_diff + strengths[i - 1] + diff;

            self.strength_base += diff;
            self.strength_threshold.push(threshold);
        }

        self.cargo_base = cargo_base;
        self.cargo_diff = cargo_diff;

        self.strength_threshold.sort_unstable();
        self.prefix_sum_threshold.clear();
        self.prefix_sum_threshold.push(0);

        let mut acc = 0;

        for threshold in self.strength_threshold.iter() {
            acc += threshold;
            self.prefix_sum_threshold.push(acc);
        }

        self.idx = Self::lower_bound(&self.strength_threshold, self.x_last);
    }

    fn calculate(&mut self, x: i64) -> (i64, i64) {
        if x > self.x_last {
            self.idx = Self::lower_bound(&self.strength_threshold, x);
        } else {
            while self.idx > 0 && self.strength_threshold[self.idx - 1] >= x {
                self.idx -= 1;
            }
        }

        self.x_last = x;

        let cost = self.strength_base + self.idx as i64 * x - self.prefix_sum_threshold[self.idx];
        let need = self.cargo_base.max(x - self.cargo_diff);

        (need, cost)
    }
}

const B: usize = 256;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cargos = vec![0; n];
    let mut strengths = vec![0; n - 1];

    for i in 0..n {
        cargos[i] = scan.token::<i64>();
    }

    for i in 0..n - 1 {
        strengths[i] = scan.token::<i64>();
    }

    let cnt_blocks = (n + B - 1) / B;
    let mut blocks = Vec::with_capacity(cnt_blocks);

    for i in 0..cnt_blocks {
        let left = i * B;
        let right = ((i + 1) * B - 1).min(n - 1);
        let mut block = Block::new(left, right);

        block.rebuild(&cargos, &strengths);
        blocks.push(block);
    }

    let mut dirty = true;
    let mut cached_need = 0;
    let mut cached_cost = 0;

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (x, y) = (scan.token::<usize>() - 1, scan.token::<i64>());

            cargos[x] += y;
            blocks[x / B].rebuild(&cargos, &strengths);
            dirty = true;
        } else {
            let k = scan.token::<i64>();

            if dirty {
                let mut need = 0;
                let mut cost = 0;

                for i in (0..blocks.len()).rev() {
                    let (need_new, cost_new) = blocks[i].calculate(need);
                    need = need_new;
                    cost += cost_new;
                }

                cached_need = need;
                cached_cost = cost;
                dirty = false;
            }

            if k < cached_need {
                writeln!(out, "-1").unwrap();
            } else {
                writeln!(out, "{cached_cost}").unwrap();
            }
        }
    }
}
