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
    let mut votes = vec![0; n];

    for i in 0..n {
        votes[i] = scan.token::<i64>();
    }

    let approve = votes.iter().filter(|&x| *x == 1).count();
    let invalid = votes.iter().filter(|&x| *x == 0).count();
    let reject = votes.iter().filter(|&x| *x == -1).count();

    writeln!(
        out,
        "{}",
        if (n % 2 == 0 && invalid >= n / 2) || (n % 2 == 1 && invalid > n / 2) {
            "INVALID"
        } else if approve > reject {
            "APPROVED"
        } else {
            "REJECTED"
        }
    )
    .unwrap();
}
