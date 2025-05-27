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

    let k = scan.token::<i64>();
    let mut sum_min = i64::MAX;
    let mut cnt_internal = k;
    let mut cnt_leaf = 1;

    for d in 1..=(k as f64).sqrt() as i64 {
        if k % d == 0 {
            let q = k / d;

            if q + d < sum_min {
                sum_min = q + d;
                cnt_internal = q;
                cnt_leaf = d;
            }
        }
    }

    writeln!(out, "{}", cnt_internal + cnt_leaf).unwrap();

    for parent in 1..cnt_internal {
        writeln!(out, "{} {}", parent, parent + 1).unwrap();
    }

    for i in 1..cnt_leaf {
        writeln!(out, "{} {}", i, cnt_internal + i).unwrap();
    }

    writeln!(out, "{} {}", cnt_internal, cnt_internal + cnt_leaf).unwrap();
}
