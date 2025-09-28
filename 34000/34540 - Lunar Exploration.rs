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

    let n = scan.token::<usize>();
    let mut robots_x = vec![0; n];
    let mut robots_y = vec![0; n];

    for i in 0..n {
        robots_x[i] = scan.token::<i64>();
        robots_y[i] = scan.token::<i64>();
    }

    let (x, y, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<char>(),
    );
    let ret = if d == 'E' {
        let sum_y = robots_y.iter().map(|&b| (b - y).abs()).sum::<i64>();

        robots_x.sort_unstable();

        let mut sum_x = 0;

        for (i, &a) in robots_x.iter().enumerate() {
            sum_x += (a - (x + i as i64)).abs();
        }

        sum_x + sum_y
    } else {
        let sum_x = robots_x.iter().map(|&a| (a - x).abs()).sum::<i64>();

        robots_y.sort_unstable();

        let mut sum_y = 0;

        for (i, &b) in robots_y.iter().enumerate() {
            sum_y += (b - (y + i as i64)).abs();
        }

        sum_x + sum_y
    };

    writeln!(out, "{ret}").unwrap();
}
