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
    let mut positions = vec![0; n];

    for i in 0..n {
        positions[i] = scan.token::<i64>();
    }

    positions.sort();

    if n % 2 == 0 {
        let pos1 = positions[n / 2 - 1];
        let pos2 = positions[n / 2];

        let total1 = positions.iter().map(|&x| (x - pos1).abs()).sum::<i64>();
        let total2 = positions.iter().map(|&x| (x - pos2).abs()).sum::<i64>();

        writeln!(out, "{}", if total1 <= total2 { pos1 } else { pos2 }).unwrap();
    } else {
        writeln!(out, "{}", positions[n / 2]).unwrap();
    }
}
