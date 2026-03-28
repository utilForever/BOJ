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

    let (y0, m0, d0) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (y1, m1, d1) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let n = scan.token::<usize>();
    let mut birth_dates = vec![(0, 0, 0); n];

    for i in 0..n {
        birth_dates[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let happy_init = y0 - birth_dates[0].0;
    let mut ret = 0;

    for (y, m, d) in birth_dates {
        let year_from = if (m, d) > (m0, d0) { y0 } else { y0 + 1 };
        let year_to = if (m, d) <= (m1, d1) { y1 } else { y1 - 1 };

        let can_happy_from = year_from.max(y + happy_init + 1);
        let can_happy_to = year_to;

        if can_happy_from <= can_happy_to {
            ret += can_happy_to - can_happy_from + 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
