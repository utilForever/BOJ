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

pub fn next_permutation(nums: &mut Vec<usize>) -> bool {
    let last_ascending = match nums.windows(2).rposition(|w| w[0] < w[1]) {
        Some(i) => i,
        None => {
            nums.reverse();
            return false;
        }
    };

    let swap_with = nums[last_ascending + 1..]
        .binary_search_by(|n| usize::cmp(&nums[last_ascending], n).then(Ordering::Less))
        .unwrap_err();
    nums.swap(last_ascending, last_ascending + swap_with);
    nums[last_ascending + 1..].reverse();

    true
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();

    for _ in 0..n {
        let m = scan.token::<usize>();
        let mut radius = vec![0.0; m];
        let mut idxes = vec![0; m];

        for i in 0..m {
            radius[i] = scan.token::<f64>();
        }

        for i in 0..m {
            idxes[i] = i;
        }

        let mut positions = vec![0.0; m];
        let mut ret = f64::MAX;

        loop {
            positions[0] = radius[idxes[0]];
            let mut positions_max = radius[idxes[0]] * 2.0;

            for i in 1..m {
                let mut val = radius[idxes[i]];

                for j in 0..i {
                    val =
                        val.max(positions[j] + (4.0 * radius[idxes[i]] * radius[idxes[j]]).sqrt());
                }

                positions[i] = val;
                positions_max = positions_max.max(positions[i] + radius[idxes[i]]);
            }

            ret = ret.min(positions_max);

            if !next_permutation(&mut idxes) {
                break;
            }
        }

        writeln!(out, "{:.3}", ret).unwrap();
    }
}
