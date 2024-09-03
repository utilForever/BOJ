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

    let mut num = 2;
    let mut cnt = 1;
    let mut ret = vec![1];

    while cnt < 100000 {
        // We only check the numbers that are multiples of 3 or 5
        if num % 3 == 0 || num % 5 == 0 {
            let mut val = num;
            let mut q3 = 0;
            let mut q5 = 0;

            // Factorize the number into 3 and 5
            while val % 3 == 0 {
                val /= 3;
                q3 += 1;
            }

            while val % 5 == 0 {
                val /= 5;
                q5 += 1;
            }

            // Remove the common factors of 3 and 5
            let min = q3.min(q5);
            q3 -= min;
            q5 -= min;

            // writeln!(out, "num = {}, q3 = {}, q5 = {}, val = {}", num, q3, q5, val).unwrap();

            // Consider a coordinate plane.
            // The coordinates (a, b) indicate that we have used 3 for a and 5 for b. 
            // => Add the points that lie on a straight line that looks like y = x + 3 * k.
            // Therefore, 3 ^ (i + 3) * 5 ^ (i + 3 * j) * x (NOTE: gcd(x, 15) = 1)
            if val % 3 == 0 || val % 5 == 0 {
                num += 1;
                continue;
            }

            if q3 % 3 != 0 || q5 % 3 != 0 {
                num += 1;
                continue;
            }
        }

        ret.push(num);
        num += 1;
        cnt += 1;
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let i = scan.token::<usize>();
        writeln!(out, "{}", ret[i - 1]).unwrap();
    }
}
