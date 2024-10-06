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
    let mut heights_cow = vec![0; n];
    let mut heights_candy_canes = vec![0; m];

    for i in 0..n {
        heights_cow[i] = scan.token::<i64>();
    }

    for i in 0..m {
        heights_candy_canes[i] = scan.token::<i64>();
    }

    for height in heights_candy_canes {
        let mut height_ate = 0;

        for i in 0..n {
            if heights_cow[i] > height_ate {
                let height_eat = (height - height_ate).min(heights_cow[i] - height_ate);
                heights_cow[i] += height_eat;
                height_ate += height_eat;
            }

            if height_ate >= height {
                break;
            }
        }
    }

    for height in heights_cow {
        writeln!(out, "{height}").unwrap();
    }
}
