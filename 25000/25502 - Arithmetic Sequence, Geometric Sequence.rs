use io::Write;
use std::{collections::HashMap, io, str};

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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut arithmetic = HashMap::new();
    let mut geometric = HashMap::new();
    let mut cnt_arithmetic_invalid = 0;
    let mut cnt_geometric_invalid = 0;

    for i in 0..n - 1 {
        let diff = nums[i + 1] - nums[i];
        let ratio = if nums[i + 1] % nums[i] == 0 {
            nums[i + 1] / nums[i]
        } else {
            -1
        };

        if diff > 0 {
            arithmetic.entry(diff).and_modify(|e| *e += 1).or_insert(1);
        } else {
            cnt_arithmetic_invalid += 1;
        }

        if ratio > 0 {
            geometric.entry(ratio).and_modify(|e| *e += 1).or_insert(1);
        } else {
            cnt_geometric_invalid += 1;
        }
    }

    for _ in 0..m {
        let (i, x) = (scan.token::<usize>() - 1, scan.token::<i64>());
        let diff_old1 = if i > 0 { nums[i] - nums[i - 1] } else { 0 };
        let diff_old2 = if i < n - 1 { nums[i + 1] - nums[i] } else { 0 };
        let ratio_old1 = if i > 0 && nums[i - 1] != 0 && nums[i] % nums[i - 1] == 0 {
            nums[i] / nums[i - 1]
        } else {
            -1
        };
        let ratio_old2 = if i < n - 1 && nums[i] != 0 && nums[i + 1] % nums[i] == 0 {
            nums[i + 1] / nums[i]
        } else {
            -1
        };
        let is_diff_invalid_old1 = diff_old1 <= 0;
        let is_diff_invalid_old2 = diff_old2 <= 0;
        let is_ratio_invalid_old1 = ratio_old1 <= 0;
        let is_ratio_invalid_old2 = ratio_old2 <= 0;

        if diff_old1 > 0 {
            arithmetic.entry(diff_old1).and_modify(|e| *e -= 1);

            if arithmetic[&diff_old1] == 0 {
                arithmetic.remove(&diff_old1);
            }
        }

        if ratio_old1 > 0 {
            geometric.entry(ratio_old1).and_modify(|e| *e -= 1);

            if geometric[&ratio_old1] == 0 {
                geometric.remove(&ratio_old1);
            }
        }

        if diff_old2 > 0 {
            arithmetic.entry(diff_old2).and_modify(|e| *e -= 1);

            if arithmetic[&diff_old2] == 0 {
                arithmetic.remove(&diff_old2);
            }
        }

        if ratio_old2 > 0 {
            geometric.entry(ratio_old2).and_modify(|e| *e -= 1);

            if geometric[&ratio_old2] == 0 {
                geometric.remove(&ratio_old2);
            }
        }

        nums[i] = x;

        if i > 0 {
            let diff_new1 = nums[i] - nums[i - 1];

            if diff_new1 > 0 {
                arithmetic
                    .entry(diff_new1)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                if is_diff_invalid_old1 {
                    cnt_arithmetic_invalid -= 1;
                }
            } else {
                if !is_diff_invalid_old1 {
                    cnt_arithmetic_invalid += 1;
                }
            }

            let ratio_new1 = if nums[i] % nums[i - 1] == 0 {
                nums[i] / nums[i - 1]
            } else {
                -1
            };

            if ratio_new1 > 0 {
                geometric
                    .entry(ratio_new1)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                if is_ratio_invalid_old1 {
                    cnt_geometric_invalid -= 1;
                }
            } else {
                if !is_ratio_invalid_old1 {
                    cnt_geometric_invalid += 1;
                }
            }
        }

        if i < n - 1 {
            let diff_new2 = nums[i + 1] - nums[i];

            if diff_new2 > 0 {
                arithmetic
                    .entry(diff_new2)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                if is_diff_invalid_old2 {
                    cnt_arithmetic_invalid -= 1;
                }
            } else {
                if !is_diff_invalid_old2 {
                    cnt_arithmetic_invalid += 1;
                }
            }

            let ratio_new2 = if nums[i + 1] % nums[i] == 0 {
                nums[i + 1] / nums[i]
            } else {
                -1
            };

            if ratio_new2 > 0 {
                geometric
                    .entry(ratio_new2)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                if is_ratio_invalid_old2 {
                    cnt_geometric_invalid -= 1;
                }
            } else {
                if !is_ratio_invalid_old2 {
                    cnt_geometric_invalid += 1;
                }
            }
        }

        let arithmetic_count = arithmetic.len();
        let geometric_count = geometric.len();

        writeln!(
            out,
            "{}",
            if cnt_arithmetic_invalid == 0 && arithmetic_count == 1 {
                "+"
            } else if cnt_geometric_invalid == 0 && geometric_count == 1 {
                "*"
            } else {
                "?"
            }
        )
        .unwrap();
    }
}
