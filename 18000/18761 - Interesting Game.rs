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

fn trim_prefix(sum: i64, len: i64, b: i64) -> i64 {
    let base = sum / len;
    let cnt = sum % len;

    if cnt <= b {
        base * (len - b)
    } else {
        sum - b * (base + 1)
    }
}

struct Game {
    n: usize,
    a: i64,
    b: usize,
    nums: Vec<i64>,
    prefix_sum: Vec<i64>,
    best: Vec<i64>,
}

impl Game {
    fn new(n: usize, a: i64, b: usize, nums: Vec<i64>) -> Self {
        Game {
            n,
            a,
            b,
            nums,
            prefix_sum: vec![0; n + 1],
            best: vec![0; n + 1],
        }
    }

    fn rebuild_prefix(&mut self) {
        self.prefix_sum[0] = 0;

        for i in 1..=self.n {
            self.prefix_sum[i] = self.prefix_sum[i - 1] + self.nums[i];
        }
    }

    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum[r] - self.prefix_sum[l - 1]
    }

    fn blocks_until(&self, idx: usize) -> usize {
        (idx + self.b - 1) / self.b
    }

    fn full_blocks_before(&self, idx: usize) -> usize {
        (idx - 1) / self.b
    }

    fn collect_from_current_state(&mut self) {
        self.nums[1..=self.n].sort_unstable_by(|a, b| b.cmp(a));
        self.rebuild_prefix();

        let mut left = 1;
        let mut right = 1;

        while right <= self.n {
            while left <= self.n {
                let blocks_before = self.blocks_until(left);
                let segment_sum = self.range_sum(left, right);
                let max_possible = segment_sum + (blocks_before as i64) * self.a;
                let need = self.nums[left] * ((right - left + 1) as i64);

                if max_possible >= need {
                    break;
                }

                left += 1;
            }

            if left == right {
                let blocks_done = self.full_blocks_before(right);
                let candidate = self.nums[right] + (blocks_done as i64) * self.a;

                self.best[0] = self.best[0].max(candidate);
                right += self.b;
                continue;
            }

            let len_segment = (right - left + 1) as i64;
            let blocks_before = self.blocks_until(left);
            let sum_segment = self.range_sum(left, right) + (blocks_before as i64) * self.a;

            let next_removed_block_end = blocks_before * self.b;
            let removed_inside = (next_removed_block_end - left + 1) as i64;

            let blocks_total = self.full_blocks_before(right);
            let remaining_blocks = blocks_total - blocks_before;
            let candidate = trim_prefix(sum_segment, len_segment, removed_inside);

            self.best[remaining_blocks] = self.best[remaining_blocks].max(candidate);
            right += self.b;
        }
    }

    fn apply_turn(&mut self, snapshot_prefix: &Vec<i64>) {
        for i in (0..=self.n).rev() {
            if i == 0 {
                let mut rest = self.a;
                let base = self.nums[1];

                for j in 1..=self.n {
                    rest -= base - self.nums[j];
                    self.nums[j] = base;
                }

                let len = self.n as i64;

                if len > 0 {
                    let add = rest / len;
                    let mut extra = rest % len;

                    for j in 1..=self.n {
                        self.nums[j] += add;
                    }

                    let mut j = 1;

                    while extra > 0 {
                        self.nums[j] += 1;
                        j += 1;
                        extra -= 1;
                    }
                }

                break;
            } else {
                let suffix_len = (self.n - i) as i64;

                if suffix_len == 0 {
                    continue;
                }

                let suffix_sum = snapshot_prefix[self.n] - snapshot_prefix[i];

                if self.nums[i] * suffix_len > suffix_sum + self.a {
                    let mut rest = self.a;

                    if i < self.n {
                        let base = self.nums[i + 1];

                        for j in (i + 1)..=self.n {
                            rest -= base - self.nums[j];
                            self.nums[j] = base;
                        }

                        let len = suffix_len;

                        if len > 0 {
                            let add = rest / len;
                            let mut extra = rest % len;

                            for j in (i + 1)..=self.n {
                                self.nums[j] += add;
                            }

                            let mut j = i + 1;

                            while extra > 0 {
                                self.nums[j] += 1;
                                j += 1;
                                extra -= 1;
                            }
                        }
                    }

                    break;
                }
            }
        }
    }

    fn set_ideal_state(&mut self) {
        let val = (self.a - 1) / (self.b as i64);
        let border = self.n - self.b;

        for i in 1..=border {
            self.nums[i] = val;
        }

        for i in (border + 1)..=self.n {
            self.nums[i] = 0;
        }
    }

    fn compress_best(&mut self) {
        let idx_max = (self.n - 1) / self.b;

        for i in (1..=idx_max).rev() {
            let len = (i * self.b + 1) as i64;
            let candidate = trim_prefix(self.best[i] + self.a, len, self.b as i64);

            self.best[i - 1] = self.best[i - 1].max(candidate);
        }
    }

    fn solve(mut self) -> i64 {
        self.collect_from_current_state();

        {
            let snapshot_prefix = self.prefix_sum.clone();
            self.apply_turn(&snapshot_prefix);

            for i in 1..=self.b {
                self.nums[i] = 0;
            }

            self.collect_from_current_state();
        }

        {
            self.set_ideal_state();
            self.collect_from_current_state();
        }

        self.compress_best();

        self.best[0] + self.a
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, a, b) = (
            scan.token::<usize>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );
        let mut nums = vec![0; n + 1];

        for i in 1..=n {
            nums[i] = scan.token::<i64>();
        }

        let game = Game::new(n, a, b, nums);

        writeln!(out, "{}", game.solve()).unwrap();
    }
}
