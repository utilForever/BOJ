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

fn is_valid_time(x: i64, y: i64) -> bool {
    x >= 0 && x <= 23 && y >= 0 && y <= 59
}

fn is_valid_date(x: i64, y: i64) -> bool {
    match x {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => y >= 1 && y <= 31,
        4 | 6 | 9 | 11 => y >= 1 && y <= 30,
        2 => y >= 1 && y <= 29,
        _ => false,
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (x, y) = (scan.token::<i64>(), scan.token::<i64>());

        writeln!(
            out,
            "{} {}",
            if is_valid_time(x, y) { "Yes" } else { "No" },
            if is_valid_date(x, y) { "Yes" } else { "No" }
        )
        .unwrap();
    }
}
