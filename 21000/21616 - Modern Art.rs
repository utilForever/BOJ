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

    let (m, n) = (scan.token::<usize>(), scan.token::<usize>());
    let mut rows = vec![false; m];
    let mut cols = vec![false; n];

    let k = scan.token::<i64>();
    let mut cnt_row = 0;
    let mut cnt_col = 0;

    for _ in 0..k {
        let (r#type, idx) = (scan.token::<char>(), scan.token::<usize>() - 1);

        if r#type == 'R' {
            rows[idx] = !rows[idx];
            cnt_row += if rows[idx] { 1 } else { -1 };
        } else {
            cols[idx] = !cols[idx];
            cnt_col += if cols[idx] { 1 } else { -1 };
        }
    }

    writeln!(
        out,
        "{}",
        cnt_row * (n as i64 - cnt_col) + cnt_col * (m as i64 - cnt_row)
    )
    .unwrap();
}
