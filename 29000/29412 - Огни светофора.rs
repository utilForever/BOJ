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

    let (g, gb, y, r, ry) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let total = g + gb + y + r + ry;
    let mut t = scan.token::<i64>();
    let q = t / total;
    let mut red = 0;
    let mut yellow = 0;
    let mut green = 0;

    t -= q * total;
    red += q * (r + ry);
    yellow += q * (y + ry);
    green += q * (g + gb / 2);

    if t >= g {
        t -= g;
        green += g;
    } else {
        green += t;
        t = 0;
    }

    if t >= gb {
        t -= gb;
        green += gb / 2;
    } else {
        green += t / 2;
        t = 0;
    }

    if t >= y {
        t -= y;
        yellow += y;
    } else {
        yellow += t;
        t = 0;
    }

    if t >= r {
        t -= r;
        red += r;
    } else {
        red += t;
        t = 0;
    }

    if t >= ry {
        red += ry;
        yellow += ry;
    } else {
        red += t;
        yellow += t;
    }

    writeln!(out, "{red} {yellow} {green}").unwrap();
}
