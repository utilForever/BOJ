use io::Write;
use std::{cmp, io, str};

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
    let mut num_square = vec![0; n + 1];

    for i in 1..=n {
        num_square[i] = usize::MAX;
    }

    for i in 1..=n {
        let sqrt = (i as f64).sqrt() as usize;
        if sqrt * sqrt == i {
            num_square[i] = 1;
        }
    }

    for i in 1..=n {
        for j in 1..=((i as f64).sqrt() as usize) {
            num_square[i] = cmp::min(num_square[i], num_square[j * j] + num_square[i - j * j]);
        }
    }

    writeln!(out, "{}", num_square[n]).unwrap();
}
