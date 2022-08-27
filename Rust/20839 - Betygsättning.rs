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

    let (x, y, z) = (
        scan.token::<i64>() as f64,
        scan.token::<i64>() as f64,
        scan.token::<i64>() as f64,
    );
    let (x_grade, y_grade, z_grade) = (
        scan.token::<i64>() as f64,
        scan.token::<i64>() as f64,
        scan.token::<i64>() as f64,
    );

    writeln!(
        out,
        "{}",
        if x_grade >= x && y_grade >= y && z_grade >= z {
            "A"
        } else if x_grade >= x / 2.0 && y_grade >= y && z_grade >= z {
            "B"
        } else if y_grade >= y && z_grade >= z {
            "C"
        } else if y_grade >= y / 2.0 && z_grade >= z / 2.0 {
            "D"
        } else {
            "E"
        }
    )
    .unwrap();
}
