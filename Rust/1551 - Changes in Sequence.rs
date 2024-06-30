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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let s = scan.token::<String>();
    let s = s.split(",").collect::<Vec<_>>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = s[i].parse::<i64>().unwrap();
    }

    for _ in 0..k {
        let mut nums_new = vec![0; nums.len() - 1];

        for i in 0..nums.len() - 1 {
            nums_new[i] = nums[i + 1] - nums[i];
        }

        nums = nums_new;
    }

    for i in 0..nums.len() {
        write!(out, "{}", nums[i]).unwrap();
        
        if i != nums.len() - 1 {
            write!(out, ",").unwrap();
        }
    }

    writeln!(out).unwrap();
}
