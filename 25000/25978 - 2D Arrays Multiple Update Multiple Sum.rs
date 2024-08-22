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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![vec![0; n + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=n {
            nums[i][j] = scan.token::<i64>();
            nums[i][j] += nums[i - 1][j] + nums[i][j - 1] - nums[i - 1][j - 1];
        }
    }

    // Using 2D imos method
    let mut sum = vec![vec![0; n + 2]; n + 2];
    let mut should_prepross = true;

    for _ in 0..m {
        let command = scan.token::<i32>();

        if command == 1 {
            let (i1, j1, i2, j2, k) = (
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
                scan.token::<i64>(),
            );

            // 2D imos method technique (using 2D prefix sum)
            sum[i1][j1] += k;
            sum[i2 + 1][j1] -= k;
            sum[i1][j2 + 1] -= k;
            sum[i2 + 1][j2 + 1] += k;
        } else {
            let (i1, j1, i2, j2) = (
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
                scan.token::<usize>() + 1,
            );

            if should_prepross {
                should_prepross = false;

                // Horizontal prefix sum
                for i in 1..=n {
                    for j in 2..=n {
                        sum[i][j] += sum[i][j - 1];
                    }
                }

                // Vertical prefix sum
                for j in 1..=n {
                    for i in 2..=n {
                        sum[i][j] += sum[i - 1][j];
                    }
                }

                // Apply the sum to the array
                let mut prefix_sum = vec![vec![0; n + 1]; n + 1];

                for i in 1..=n {
                    for j in 1..=n {
                        prefix_sum[i][j] = prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                            - prefix_sum[i - 1][j - 1]
                            + sum[i][j];
                        nums[i][j] += prefix_sum[i][j];
                    }
                }
            }

            // 2D prefix sum technique
            writeln!(
                out,
                "{}",
                nums[i2][j2] - nums[i1 - 1][j2] - nums[i2][j1 - 1] + nums[i1 - 1][j1 - 1]
            )
            .unwrap();
        }
    }
}
