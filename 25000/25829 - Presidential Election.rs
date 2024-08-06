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

    let n = scan.token::<i64>();
    let (mut majority_first, mut electoral_first) = (0, 0);
    let (mut majority_second, mut electoral_second) = (0, 0);

    for _ in 0..n {
        let (e, v1, v2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        majority_first += v1;
        majority_second += v2;

        if v1 > v2 {
            electoral_first += e;
        } else if v1 < v2 {
            electoral_second += e;
        }
    }

    writeln!(
        out,
        "{}",
        if majority_first > majority_second && electoral_first > electoral_second {
            1
        } else if majority_first < majority_second && electoral_first < electoral_second {
            2
        } else {
            0
        }
    )
    .unwrap();
}
