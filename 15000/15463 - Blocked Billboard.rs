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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let mut billboard = vec![vec![0; 2001]; 2001];

    let (x1, y1, x2, y2) = (
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
    );

    for x in x1..x2 {
        for y in y1..y2 {
            billboard[x as usize][y as usize] = 1;
        }
    }

    let (x1, y1, x2, y2) = (
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
    );

    for x in x1..x2 {
        for y in y1..y2 {
            billboard[x as usize][y as usize] = 1;
        }
    }

    let (x1, y1, x2, y2) = (
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
        scan.token::<i64>() + 1000,
    );

    for x in x1..x2 {
        for y in y1..y2 {
            billboard[x as usize][y as usize] = 0;
        }
    }

    let mut ret = 0;

    for x in 0..=2000 {
        for y in 0..=2000 {
            if billboard[x][y] == 1 {
                ret += 1;
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
