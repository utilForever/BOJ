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
                let slice: &str = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut fibonacci = vec![vec![0_i64; 2]; 41];
    fibonacci[0][0] = 1;
    fibonacci[0][1] = 0;
    fibonacci[1][0] = 0;
    fibonacci[1][1] = 1;

    for i in 2..=40 {
        fibonacci[i][0] = fibonacci[i - 1][0] + fibonacci[i - 2][0];
        fibonacci[i][1] = fibonacci[i - 1][1] + fibonacci[i - 2][1];
    }

    let n = scan.token::<usize>();

    writeln!(out, "{} {}", fibonacci[n][1], n - 2).unwrap();
}
