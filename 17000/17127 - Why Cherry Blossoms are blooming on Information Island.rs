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
    let mut flowers = vec![0; n];

    for i in 0..n {
        flowers[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in 0..n - 3 {
        for j in i + 1..n - 2 {
            for k in j + 1..n - 1 {
                let val_i = flowers[0..=i].iter().fold(1, |acc, x| acc * x);
                let val_j = flowers[i + 1..=j].iter().fold(1, |acc, x| acc * x);
                let val_k = flowers[j + 1..=k].iter().fold(1, |acc, x| acc * x);
                let val_l = flowers[k + 1..n].iter().fold(1, |acc, x| acc * x);
                let sum = val_i + val_j + val_k + val_l;

                ret = ret.max(sum);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
