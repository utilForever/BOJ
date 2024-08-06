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

    let n = scan.token::<i64>();

    writeln!(
        out,
        "{}",
        if n == 1 {
            1
        } else if n < 10 {
            2
        } else {
            let mut ret = 1;
            let mut cnt = n / 10;
            let mut offset = 0;

            while cnt > 0 {
                if cnt >= 9 * 10i64.pow(offset as u32) {
                    ret += 9 * 10i64.pow(offset as u32) * (offset + 2);
                    cnt -= 9 * 10i64.pow(offset as u32);
                    offset += 1;
                } else {
                    ret += cnt * (offset + 2);
                    cnt = 0;
                }
            }

            if n % 10 != 0 {
                ret += (n as f64).log10().floor() as i64 + 1;
            }

            ret
        }
    )
    .unwrap();
}
