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

    let (a, b) = (
        scan.token::<String>().chars().collect::<Vec<_>>(),
        scan.token::<String>().chars().collect::<Vec<_>>(),
    );
    let (a_col, a_row) = (a[0] as i64 - 'a' as i64, a[1] as i64 - '1' as i64);
    let (b_col, b_row) = (b[0] as i64 - 'a' as i64, b[1] as i64 - '1' as i64);

    let diff_col = (a_col - b_col).abs();
    let diff_row = (a_row - b_row).abs();

    writeln!(out, "{} {}", diff_col.min(diff_row), diff_col.max(diff_row)).unwrap();
}
