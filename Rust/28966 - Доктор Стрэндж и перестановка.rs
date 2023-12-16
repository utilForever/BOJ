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
    let mut nums = vec![0; n + 1];

    for i in 1..=n {
        nums[i] = scan.token::<i64>();
    }

    if n == 2 {
        if nums[1] % 2 == 0 && nums[2] % 2 == 1 {
            writeln!(out, "1 2").unwrap();
        } else {
            writeln!(out, "-1 -1").unwrap();
        }
    } else {
        let mut cnt_wrong_odd = 0;
        let mut cnt_wrong_even = 0;

        for i in 1..=n {
            if i % 2 == 1 && nums[i] % 2 == 0 {
                cnt_wrong_odd += 1;
            } else if i % 2 == 0 && nums[i] % 2 == 1 {
                cnt_wrong_even += 1;
            }
        }

        if cnt_wrong_odd == 0 && cnt_wrong_even == 0 {
            writeln!(out, "1 3").unwrap();
        } else if cnt_wrong_odd == 1 && cnt_wrong_even == 1 {
            let pos_wrong_odd = (1..=n).find(|&i| i % 2 == 1 && nums[i] % 2 == 0).unwrap();
            let pos_wrong_even = (1..=n).find(|&i| i % 2 == 0 && nums[i] % 2 == 1).unwrap();

            writeln!(out, "{pos_wrong_odd} {pos_wrong_even}").unwrap();
        } else {
            writeln!(out, "-1 -1").unwrap();
        }
    }
}
