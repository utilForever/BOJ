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

    let mut triangle1 = [0; 3];
    let mut triangle2 = [0; 3];

    for i in 0..3 {
        triangle1[i] = scan.token::<i64>();
    }

    for i in 0..3 {
        triangle2[i] = scan.token::<i64>();
    }

    triangle1.sort();
    triangle2.sort();

    writeln!(
        out,
        "{}",
        if triangle1 == triangle2
            && triangle1[0].pow(2) + triangle1[1].pow(2) == triangle1[2].pow(2)
        {
            "YES"
        } else {
            "NO"
        }
    )
    .unwrap();
}
