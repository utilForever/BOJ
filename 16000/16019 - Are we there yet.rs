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

    let (a, b, c, d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = [[0; 5]; 5];

    ret[0][1] = a;
    ret[0][2] = a + b;
    ret[0][3] = a + b + c;
    ret[0][4] = a + b + c + d;
    ret[1][0] = a;
    ret[1][2] = b;
    ret[1][3] = b + c;
    ret[1][4] = b + c + d;
    ret[2][0] = a + b;
    ret[2][1] = b;
    ret[2][3] = c;
    ret[2][4] = c + d;
    ret[3][0] = a + b + c;
    ret[3][1] = b + c;
    ret[3][2] = c;
    ret[3][4] = d;
    ret[4][0] = a + b + c + d;
    ret[4][1] = b + c + d;
    ret[4][2] = c + d;
    ret[4][3] = d;

    for i in 0..5 {
        for j in 0..5 {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
