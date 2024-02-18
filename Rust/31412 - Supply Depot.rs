use io::Write;
use std::{cmp::Ordering, io, str};

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

pub fn prev_permutation(nums: &mut Vec<i64>) -> bool {
    let first_ascending = match nums.windows(2).rposition(|w| w[0] > w[1]) {
        Some(i) => i,
        None => {
            return false;
        }
    };

    let swap_with = nums[first_ascending + 1..]
        .binary_search_by(|n| i64::cmp(n, &nums[first_ascending]).then(Ordering::Greater))
        .unwrap_err();
    nums.swap(first_ascending, first_ascending + swap_with);
    nums[first_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut boxes = vec![0; n];
    let mut prefix_sum = vec![0; n + 1];
    let mut soldiers = vec![0; m];

    for i in 0..n {
        boxes[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i - 1] + boxes[i - 1];
    }

    for i in 0..m {
        soldiers[i] = scan.token::<i64>();
    }

    soldiers.sort_by(|a, b| b.cmp(a));

    let check = |soldiers: &mut Vec<i64>, limit: i64| -> bool {
        loop {
            let mut curr = 0;

            for i in 0..m {
                let mut left = curr;
                let mut right = n + 1;

                while left + 1 < right {
                    let mid = (left + right) / 2;
                    let diff = prefix_sum[mid] - prefix_sum[curr];
                    let val = if diff % soldiers[i] == 0 {
                        diff / soldiers[i]
                    } else {
                        diff / soldiers[i] + 1
                    };

                    if val <= limit {
                        left = mid;
                    } else {
                        right = mid;
                    }
                }

                if curr == left {
                    break;
                }

                if left == n {
                    return true;
                }

                curr = left;
            }

            if !prev_permutation(soldiers) {
                break;
            }
        }

        false
    };

    let mut left = 0;
    let mut right = prefix_sum[n];

    while left + 1 < right {
        let mut soldiers_clone = soldiers.clone();
        let mid = (left + right) / 2;

        if check(&mut soldiers_clone, mid) {
            right = mid;
        } else {
            left = mid;
        }
    }

    writeln!(out, "{right}").unwrap();
}
