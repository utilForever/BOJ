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

pub fn prev_permutation(nums: &mut Vec<i32>) -> bool {
    let first_ascending = match nums.windows(2).rposition(|w| w[0] > w[1]) {
        Some(i) => i,
        None => {
            return false;
        }
    };

    let swap_with = nums[first_ascending + 1..]
        .binary_search_by(|n| i32::cmp(n, &nums[first_ascending]).then(Ordering::Greater))
        .unwrap_err();
    nums.swap(first_ascending, first_ascending + swap_with);
    nums[first_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut abilities = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            abilities[i][j] = scan.token::<i64>();
        }
    }

    let mut permutation = vec![0; n];
    let mut score_start = 0;
    let mut score_link = 0;

    for i in 0..n / 2 {
        permutation[i] = 1;
    }

    for i in 0..n {
        for j in 0..n {
            if permutation[i] == 0 && permutation[j] == 0 {
                score_start += abilities[i][j];
            } else if permutation[i] == 1 && permutation[j] == 1 {
                score_link += abilities[i][j];
            }
        }
    }

    let mut ret = (score_start - score_link).abs();

    loop {
        let is_exist = prev_permutation(&mut permutation);

        if !is_exist {
            break;
        }

        let mut score_start = 0;
        let mut score_link = 0;

        for i in 0..n {
            for j in 0..n {
                if permutation[i] == 0 && permutation[j] == 0 {
                    score_start += abilities[i][j];
                } else if permutation[i] == 1 && permutation[j] == 1 {
                    score_link += abilities[i][j];
                }
            }
        }

        ret = ret.min((score_start - score_link).abs());
    }

    writeln!(out, "{ret}").unwrap();
}
