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

    let n = scan.token::<usize>();
    let mut radius = vec![0.0; n];
    let mut idxes = vec![0; n];

    for i in 0..n {
        radius[i] = scan.token::<f64>();
    }

    for i in 0..n {
        idxes[i] = i;
    }

    let mut positions = vec![0.0; n];
    let mut ret = f64::MAX;

    loop {
        for i in 1..n {
            let mut val = 0.0f64;

            for j in 0..i {
                val = val.max(positions[j] + (4.0 * radius[idxes[j]] * radius[idxes[i]]).sqrt());
            }

            positions[i] = val;
        }

        ret = ret.min(positions[n - 1]);

        if !next_permutation(&mut idxes) {
            break;
        }
    }

    writeln!(out, "{:.9}", ret).unwrap();
}
