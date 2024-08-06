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

pub fn next_permutation(nums: &mut Vec<i64>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| i64::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut comparators = vec![' '; n];

    for i in 0..n {
        comparators[i] = scan.token::<char>();
    }

    let mut nums = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut ret = Vec::new();

    loop {
        let mut is_valid = true;

        for i in 0..n {
            match comparators[i] {
                '<' => {
                    if nums[i] >= nums[i + 1] {
                        is_valid = false;
                        break;
                    }
                }
                '>' => {
                    if nums[i] <= nums[i + 1] {
                        is_valid = false;
                        break;
                    }
                }
                _ => unreachable!(),
            }
        }

        if is_valid {
            ret.push(nums.clone());
        }

        if !next_permutation(&mut nums) {
            break;
        }
    }

    ret.sort();

    for i in 0..=n {
        write!(out, "{}", ret[ret.len() - 1][i]).unwrap();
    }

    writeln!(out).unwrap();

    for i in 0..=n {
        write!(out, "{}", ret[0][i]).unwrap();
    }

    writeln!(out).unwrap();
}
