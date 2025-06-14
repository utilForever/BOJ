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

fn calc_nums(nums: &mut Vec<i64>, mut n: i64, cnt: i64) {
    while n > 0 {
        nums[(n % 10) as usize] += cnt;
        n /= 10;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut end = scan.token::<i64>();
    let mut nums = vec![0; 10];
    let mut start = 1;
    let mut multiplier = 1;

    while start <= end {
        while start % 10 != 0 && start <= end {
            calc_nums(&mut nums, start, multiplier);
            start += 1;
        }

        if start > end {
            break;
        }

        while end % 10 != 9 && start <= end {
            calc_nums(&mut nums, end, multiplier);
            end -= 1;
        }

        let cnt = (end / 10) - (start / 10) + 1;
        for i in 0..10 {
            nums[i] += cnt * multiplier;
        }

        start /= 10;
        end /= 10;
        multiplier *= 10;
    }

    for i in 0..10 {
        write!(out, "{} ", nums[i]).unwrap();
    }

    writeln!(out).unwrap();
}
