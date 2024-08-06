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

    let x = scan.token::<String>();
    let x = x
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();

    if x[0] >= x[1] || x[x.len() - 2] <= x[x.len() - 1] {
        writeln!(out, "NON ALPSOO").unwrap();
        return;
    }

    let mut ret = true;
    let mut derivative = x[1] - x[0];

    for i in 2..x.len() {
        let d = x[i] - x[i - 1];

        if d == 0 {
            ret = false;
            break;
        }

        if d > 0 && derivative > 0 && d != derivative {
            ret = false;
            break;
        }

        if d < 0 && derivative < 0 && d != derivative {
            ret = false;
            break;
        }

        derivative = d;
    }

    writeln!(out, "{}", if ret { "ALPSOO" } else { "NON ALPSOO" }).unwrap();
}
