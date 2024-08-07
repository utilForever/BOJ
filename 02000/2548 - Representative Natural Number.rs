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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    nums.sort();

    if n % 2 == 1 {
        writeln!(out, "{}", nums[n / 2]).unwrap();
    } else {
        let candidate1 = nums[n / 2 - 1];
        let candidate2 = nums[n / 2];
        let sum1 = nums.iter().map(|&x| (x - candidate1).abs()).sum::<i64>();
        let sum2 = nums.iter().map(|&x| (x - candidate2).abs()).sum::<i64>();

        writeln!(
            out,
            "{}",
            if sum1 <= sum2 { candidate1 } else { candidate2 }
        )
        .unwrap();
    }
}
