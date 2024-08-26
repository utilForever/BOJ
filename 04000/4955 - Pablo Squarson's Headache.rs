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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let n = scan.token::<usize>();

        if n == 0 {
            break;
        }

        let mut positions = vec![(0, 0); n];

        for i in 1..n {
            let (ni, di) = (scan.token::<usize>(), scan.token::<i64>());
            let mut pos = positions[ni];

            match di {
                0 => pos.1 -= 1,
                1 => pos.0 += 1,
                2 => pos.1 += 1,
                3 => pos.0 -= 1,
                _ => unreachable!(),
            }

            positions[i] = pos;
        }

        let y_max = positions.iter().map(|(y, _)| y).max().unwrap();
        let y_min = positions.iter().map(|(y, _)| y).min().unwrap();
        let x_max = positions.iter().map(|(_, x)| x).max().unwrap();
        let x_min = positions.iter().map(|(_, x)| x).min().unwrap();

        writeln!(out, "{} {}", x_max - x_min + 1, y_max - y_min + 1).unwrap();
    }
}
