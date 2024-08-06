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

fn check_condition(outlets: &Vec<i64>, num_outlet: usize, dist: i64, num_choose: usize) -> bool {
    let mut ret = 1;
    let mut base = outlets[0];

    for i in 1..num_outlet {
        if outlets[i] - base >= dist {
            ret += 1;
            base = outlets[i];
        }
    }

    ret >= num_choose
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, s) = (scan.token::<usize>(), scan.token::<usize>());
        let mut outlets = vec![0; n];

        for i in 0..n {
            outlets[i] = scan.token::<i64>();
        }

        outlets.sort();

        let mut left = 1;
        let mut right = outlets[n - 1] - outlets[0];
        let mut ret = 0;

        while left <= right {
            let mid = (left + right) / 2;

            if check_condition(&outlets, n, mid, s) {
                ret = ret.max(mid);
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        writeln!(out, "{}", ret).unwrap();
    }
}
