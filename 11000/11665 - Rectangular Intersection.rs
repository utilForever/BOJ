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

    let n = scan.token::<i64>();
    let mut x_min = 0;
    let mut x_max = 1001;
    let mut y_min = 0;
    let mut y_max = 1001;
    let mut z_min = 0;
    let mut z_max = 1001;

    for _ in 0..n {
        let (x1, y1, z1, x2, y2, z2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        x_min = x_min.max(x1);
        x_max = x_max.min(x2);
        y_min = y_min.max(y1);
        y_max = y_max.min(y2);
        z_min = z_min.max(z1);
        z_max = z_max.min(z2);
    }

    let x = x_max - x_min;
    let y = y_max - y_min;
    let z = z_max - z_min;

    if x <= 0 || y <= 0 || z <= 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    writeln!(out, "{}", x * y * z).unwrap();
}
