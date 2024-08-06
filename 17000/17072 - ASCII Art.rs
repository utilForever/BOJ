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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut image = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let (r, g, b) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            image[i][j] = 2126 * r + 7152 * g + 722 * b;
        }
    }

    for i in 0..n {
        for j in 0..m {
            write!(
                out,
                "{}",
                match image[i][j] {
                    0..=509_999 => '#',
                    510_000..=1_019_999 => 'o',
                    1_020_000..=1_529_999 => '+',
                    1_530_000..=2_039_999 => '-',
                    _ => '.',
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    }
}
