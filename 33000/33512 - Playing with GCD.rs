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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn compute_diff(freq: &Vec<usize>, max: usize) -> i64 {
    let mut count = vec![0; max + 1];

    for d in 1..=max {
        let mut cnt = 0;

        for m in (d..=max).step_by(d) {
            cnt += freq[m];
        }

        count[d] = cnt;
    }

    let mut f = vec![0_usize; max + 1];

    for d in 1..=max {
        let cnt = count[d];

        if cnt >= 2 {
            f[d] = cnt * (cnt - 1) / 2;
        }
    }

    let mut g = vec![0_usize; max + 1];

    for d in (1..=max).rev() {
        let mut sum = 0;
        let mut m = d * 2;

        while m <= max {
            sum += g[m];
            m += d;
        }

        g[d] = f[d].saturating_sub(sum);
    }

    let mut gcd_min = None;
    let mut gcd_max = None;

    for d in 1..=max {
        if g[d] > 0 {
            if gcd_min.is_none() {
                gcd_min = Some(d);
            }

            gcd_max = Some(d);
        }
    }

    if let (Some(gcd_min), Some(gcd_max)) = (gcd_min, gcd_max) {
        (gcd_max as i64) - (gcd_min as i64)
    } else {
        0
    }
}

fn process(nums: &Vec<i64>) -> i64 {
    let max = *nums.iter().max().unwrap() as usize;
    let mut freq = vec![0; max + 1];

    for &num in nums.iter() {
        freq[num as usize] += 1;
    }

    compute_diff(&freq, max)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let ret = if k == 0 {
        // If k is 0, we can't add any number to the array
        // So, we just compare the gcd of all numbers in the array
        process(&nums)
    } else if k == 1 {
        // If k is 1, we can add 1 or max(nums) to the array
        // Case 1: Add 1 to the array
        let mut candidate1 = nums.clone();
        candidate1.push(1);
        let diff1 = process(&candidate1);

        // Case 2: Add max(nums) to the array
        let mut candidate2 = nums.clone();
        candidate2.push(*nums.iter().max().unwrap());
        let diff2 = process(&candidate2);

        diff1.max(diff2)
    } else if k == 2 {
        // If k is 2, we can add two cases to the array
        // Case 1: Add 1, max(nums) to the array
        let mut candidate1 = nums.clone();
        candidate1.push(1);
        candidate1.push(*nums.iter().max().unwrap());

        let diff1 = process(&candidate1);

        // Case 2: Add from (99991, 99991) to (100000, 100000) to the array
        // Because 99991 is a prime number, the gcd of any two numbers in the array is 1 except for (99991, 99991)
        // And the gcd of (99991, 99991) is 99991
        let mut diff2 = 0;

        for cand in 99991..=100000 {
            let mut candidate = nums.clone();
            candidate.push(cand);
            candidate.push(cand);

            diff2 = diff2.max(process(&candidate));
        }

        diff1.max(diff2)
    } else {
        // If k is greater than 2, we can add 1, 10^5, 10^5, ... to the array
        // => min(gcd(a_i, a_j)) = 1, max(gcd(a_i, a_j)) = 10^5
        // => diff = 10^5 - 1 = 99999
        99999
    };

    writeln!(out, "{ret}").unwrap();
}
