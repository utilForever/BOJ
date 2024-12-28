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

    let (p3, p4, p0) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let total = p3 + p4 + p0;

    let t3_min = (p3 + 2) / 3;
    let t3_max = total / 3;
    let mut ret = None;

    for t3 in t3_min..=t3_max {
        let left = total - 3 * t3;

        if left < 0 {
            break;
        }

        if left % 4 != 0 {
            continue;
        }

        let t4 = left / 4;

        if 4 * t4 >= p4 {
            ret = Some((t3, t4));
            break;
        }
    }

    match ret {
        Some((t3, t4)) => writeln!(out, "{t3} {t4}").unwrap(),
        None => writeln!(out, "-1").unwrap(),
    }
}
