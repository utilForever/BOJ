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
    let mut dice1 = vec![0; n];
    let mut dice2 = vec![0; n];
    let mut ret = 0;

    for i in 0..n {
        dice1[i] = scan.token::<i64>();
    }

    for i in 0..n {
        dice2[i] = scan.token::<i64>();
    }

    for i in 0..n {
        for j in 0..n {
            if dice1[i] > dice2[j] {
                ret += 1;
            } else if dice1[i] < dice2[j] {
                ret -= 1;
            }
        }
    }

    writeln!(
        out,
        "{}",
        match ret.cmp(&0) {
            std::cmp::Ordering::Greater => "first",
            std::cmp::Ordering::Less => "second",
            std::cmp::Ordering::Equal => "tie",
        }
    )
    .unwrap();
}
