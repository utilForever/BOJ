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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = vec![0; n];

        for i in 0..n {
            nums[i] = scan.token::<i64>();
        }

        nums.sort();

        let mut ret = vec![0; n];
        let mut idx = 0;
        let mut left = 0;
        let mut right = n - 1;

        while left <= right {
            ret[left] = nums[idx];
            idx += 1;
            left += 1;

            if left > right {
                break;
            }

            ret[right] = nums[idx];
            idx += 1;
            right -= 1;
        }

        ret.push(ret[0]);

        writeln!(
            out,
            "{}",
            ret.windows(2).map(|x| (x[0] - x[1]).abs()).max().unwrap()
        )
        .unwrap();
    }
}
