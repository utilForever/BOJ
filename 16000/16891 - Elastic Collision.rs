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

    let n = scan.token::<f64>().powi(2);
    let mut ret = 0;

    let mut v_a = 0.0;
    let mut v_b = -10.0;

    loop {
        if v_a >= 0.0 && v_b >= 0.0 && v_a <= v_b {
            break;
        }

        ret += 1;

        let va_new = (1.0 - n) / (1.0 + n) * v_a + 2.0 * n / (1.0 + n) * v_b;
        let vb_new = 2.0 / (1.0 + n) * v_a - (1.0 - n) / (1.0 + n) * v_b;

        v_a = va_new;
        v_b = vb_new;

        if v_a < 0.0 {
            ret += 1;
            v_a *= -1.0;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
