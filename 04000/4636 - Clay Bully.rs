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

    loop {
        let n = scan.token::<i64>();

        if n == -1 {
            break;
        }

        let n = n as usize;
        let mut blocks = vec![(0, String::new()); n];

        for i in 0..n {
            let (x, y, z, name) = (
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
                scan.token::<String>(),
            );
            blocks[i] = (x * y * z, name);
        }

        blocks.sort_by(|a, b| a.0.cmp(&b.0));

        writeln!(out, "{} took clay from {}.", blocks[n - 1].1, blocks[0].1).unwrap();
    }
}
