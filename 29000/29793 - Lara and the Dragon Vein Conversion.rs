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

pub fn next_permutation(nums: &mut Vec<char>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| char::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, h) = (scan.token::<usize>(), scan.token::<i64>());
    let s = scan.token::<String>();
    let mut s = s.chars().collect::<Vec<_>>();

    if h == 1 {
        writeln!(out, "0").unwrap();
    } else if h == 2 {
        let mut ret = 0;

        for i in 1..n {
            if s[i] == s[i - 1] {
                s[i] = 'A';
                ret += 1;
            }
        }

        writeln!(out, "{ret}").unwrap();
    } else if h == 3 || n <= 3 {
        let mut ret = i64::MAX;
        let mut dragon_veins = vec!['R', 'S', 'W'];

        loop {
            let mut val = 0;

            for i in 0..n {
                if s[i] != dragon_veins[i % 3] {
                    val += 1;
                }
            }

            ret = ret.min(val);

            if !next_permutation(&mut dragon_veins) {
                break;
            }
        }

        writeln!(out, "{ret}").unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }
}
