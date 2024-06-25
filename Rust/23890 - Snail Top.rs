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

    let r = scan.token::<i64>();

    // Solution
    // Let the point of axis is (a, b). The radius is r = sqrt(a^2 + (b + R)^2)
    // If the point of axis is on the circle, a^2 + b^2 = R^2
    // The value of a can be single and the value of b can be multiple (two values)
    // => The first quadrant and the fourth quadrant
    // If the multiple answer exists, we should output the maximum value of b
    // => Therefore, b has to be on the fourth quadrant
    // sqrt(a^2 + (b + R)^2) = sqrt(a^2 + (sqrt(R^2 - a^2) + R)^2)
    //                       = sqrt(a^2 + R^2 - a^2 + 2 * R * sqrt(R^2 - a^2) + R^2)
    //                       = sqrt(2 * R^2 + 2 * R * sqrt(R^2 - a^2))
    // By increasing the value of a, the value of r is decreasing
    // Therefore, the better the value of a is smaller, the better the value of b is larger
    let mut a = ((2 * r - 1) as f64).sqrt() as i64;
    let b = r - 1;

    // Can't be on the circle
    if a * a + b * b == r * r {
        a -= 1;
    }

    writeln!(out, "{a} {b}").unwrap();
}
