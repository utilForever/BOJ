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
    let mut first = vec![0; n];
    let mut second = vec![0; n];
    let mut third = vec![0; n];
    let mut ret = vec![0; n];

    for i in 0..n {
        (first[i], second[i], third[i]) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    for i in 0..n {
        ret[i] += if first
            .iter()
            .enumerate()
            .find(|&(j, &x)| i != j && x == first[i])
            .is_some()
        {
            0
        } else {
            first[i]
        };
        ret[i] += if second
            .iter()
            .enumerate()
            .find(|&(j, &x)| i != j && x == second[i])
            .is_some()
        {
            0
        } else {
            second[i]
        };
        ret[i] += if third
            .iter()
            .enumerate()
            .find(|&(j, &x)| i != j && x == third[i])
            .is_some()
        {
            0
        } else {
            third[i]
        };
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
