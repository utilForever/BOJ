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

    if n == 1 {
        writeln!(out, "4").unwrap();
        return;
    }

    let mut nums = vec![0i64; n + 1];
    nums[1] = 1;
    nums[2] = 1;

    for i in 2..=n {
        nums[i] = nums[i - 1] + nums[i - 2];
    }

    let mut width = 1;
    let mut height = 0;

    for i in 1..=n {
        if i % 2 == 0 {
            width += nums[i];
        } else {
            height += nums[i];
        }
    }

    writeln!(out, "{}", 2 * (width + height)).unwrap();
}
