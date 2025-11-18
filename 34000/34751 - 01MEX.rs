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

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut cnt_all_zero = 0;
    let mut cnt_all_one = 0;
    let mut prev = nums[0];
    let mut len = 1;

    for i in 1..n {
        if nums[i] == prev {
            len += 1;
        } else {
            if prev == 0 {
                cnt_all_zero += len * (len + 1) / 2;
            } else {
                cnt_all_one += len * (len + 1) / 2;
            }

            prev = nums[i];
            len = 1;
        }
    }

    if prev == 0 {
        cnt_all_zero += len * (len + 1) / 2;
    } else {
        cnt_all_one += len * (len + 1) / 2;
    }

    // all cases = all zero cases + all one cases + mixed cases
    // mixed cases = all cases - all zero cases - all one cases
    // ret = 2 * mixed cases + all zero cases
    //     = 2 * (all cases - all zero cases - all one cases) + all zero cases
    //     = 2 * all cases - all zero cases - 2 * all one case
    writeln!(out, "{}", n * (n + 1) - cnt_all_zero - 2 * cnt_all_one).unwrap();
}
