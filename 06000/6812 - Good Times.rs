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

    let t = scan.token::<i64>();
    let convert = |t: i64| -> i64 {
        let mut ret = if t < 0 {
            t + 2400
        } else if t > 2400 {
            t - 2400
        } else {
            t
        };

        if ret % 100 >= 60 {
            ret = 100 * (ret / 100 + 1) + ret % 100 - 60;
        }

        if ret >= 2400 {
            ret -= 2400;
        }

        ret
    };

    writeln!(out, "{t} in Ottawa").unwrap();
    writeln!(out, "{} in Victoria", convert(t - 300)).unwrap();
    writeln!(out, "{} in Edmonton", convert(t - 200)).unwrap();
    writeln!(out, "{} in Winnipeg", convert(t - 100)).unwrap();
    writeln!(out, "{t} in Toronto").unwrap();
    writeln!(out, "{} in Halifax", convert(t + 100)).unwrap();
    writeln!(out, "{} in St. John's", convert(t + 130)).unwrap();
}
