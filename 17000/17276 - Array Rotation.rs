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

fn rotate(nums: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = nums.len();
    let mid = n / 2;
    let mut ret = nums.clone();

    for i in 0..n {
        ret[i][mid] = nums[i][i];
        ret[i][n - 1 - i] = nums[i][mid];
        ret[mid][n - 1 - i] = nums[i][n - 1 - i];
        ret[i][i] = nums[mid][i];
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, d) = (scan.token::<usize>(), scan.token::<i64>());
        let mut nums = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                nums[i][j] = scan.token::<i64>();
            }
        }

        let cnt_rotate = (d / 45).rem_euclid(8);

        for _ in 0..cnt_rotate {
            nums = rotate(&nums);
        }

        for i in 0..n {
            for j in 0..n {
                write!(out, "{} ", nums[i][j]).unwrap();
            }

            writeln!(out).unwrap();
        }
    }
}
