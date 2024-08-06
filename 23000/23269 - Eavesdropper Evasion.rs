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

fn knapsack(nums: &Vec<i64>, target: i64) -> i64 {
    let mut a = 0;
    let mut b = 0;

    while b < nums.len() && a + nums[b] <= target {
        a += nums[b];
        b += 1;
    }

    if b == nums.len() {
        return a;
    }

    let max = *nums.iter().max().unwrap();
    let mut v = vec![-1; 2 * max as usize];

    v[(a + max - target) as usize] = b as i64;

    for i in b..nums.len() {
        let u = v.clone();

        for j in 0..max {
            v[(j + nums[i]) as usize] = (v[(j + nums[i]) as usize]).max(u[j as usize]);
        }

        for j in (max + 1..2 * max).rev() {
            for k in u[j as usize].max(0)..v[j as usize] {
                v[(j - nums[k as usize]) as usize] = v[(j - nums[k as usize]) as usize].max(k);
            }
        }
    }

    a = target;

    while v[(a + max - target) as usize] < 0 {
        a -= 1;
    }

    a
}

// Reference: Pisinger 1999, "Linear Time Algorithms for Knapsack Problems with Bounded Weights"
// Reference: https://github.com/kth-competitive-programming/kactl/blob/main/content/various/FastKnapsack.h
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, x) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = Vec::with_capacity(n);
    let mut ret = 0;

    for _ in 0..n {
        let t = scan.token::<i64>();
        ret = ret.max(t);

        if t > x {
            continue;
        }

        nums.push(x - t + 1);
    }

    if nums.len() <= 2 {
        writeln!(out, "{ret}").unwrap();
        return;
    }

    if nums.len() <= 4 {
        writeln!(out, "{}", ret.max(x + 1)).unwrap();
        return;
    }

    nums.sort();
    nums.truncate(nums.len() - 4);

    let sum = nums.iter().sum::<i64>();
    let target = sum / 2;
    let val = knapsack(&nums, target);

    writeln!(out, "{}", ret.max(val.max(sum - val) + x + 1)).unwrap();
}
