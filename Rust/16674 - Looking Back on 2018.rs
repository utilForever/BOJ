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

    let mut n = scan.token::<i64>();
    let mut nums = [0; 10];

    while n > 0 {
        nums[(n % 10) as usize] += 1;
        n /= 10;
    }

    if nums
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx != 2 && *idx != 0 && *idx != 1 && *idx != 8)
        .any(|(_, &x)| x > 0)
    {
        writeln!(out, "0").unwrap();
    } else if nums
        .iter()
        .enumerate()
        .filter(|(idx, _)| *idx == 2 || *idx == 0 || *idx == 1 || *idx == 8)
        .any(|(_, &x)| x == 0)
    {
        writeln!(out, "1").unwrap();
    } else {
        let nums_filtered = nums
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx == 2 || *idx == 0 || *idx == 1 || *idx == 8)
            .map(|(_, &x)| x)
            .collect::<Vec<_>>();

        if nums_filtered.windows(2).all(|x| x[0] == x[1]) {
            writeln!(out, "8").unwrap();
        } else {
            writeln!(out, "2").unwrap();
        }
    }
}
