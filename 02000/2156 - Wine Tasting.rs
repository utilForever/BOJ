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
    let mut wines = vec![0; 10001];
    let mut wines_max = vec![0; 10001];

    for i in 1..=n {
        wines[i] = scan.token::<i64>();
    }

    wines_max[1] = wines[1];
    wines_max[2] = wines[1] + wines[2];

    for i in 3..=n {
        wines_max[i] = *vec![
            wines_max[i - 1],
            wines_max[i - 2] + wines[i],
            wines_max[i - 3] + wines[i - 1] + wines[i],
        ]
        .iter()
        .max()
        .unwrap();
    }

    writeln!(out, "{}", wines_max[n]).unwrap();
}
