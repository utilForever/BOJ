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

    loop {
        let (a, b) = (scan.token::<String>(), scan.token::<String>());

        if a == "00:00" && b == "00:00" {
            break;
        }

        let (a, b) = (
            a.split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
            b.split(':')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>(),
        );
        let (a, b) = (a[0] * 60 + a[1], b[0] * 60 + b[1]);
        let is_exceed = a + b >= 24 * 60;

        if is_exceed {
            writeln!(
                out,
                "{:02}:{:02} +{}",
                (a + b) / 60 % 24,
                (a + b) % 60,
                (a + b) / (24 * 60)
            )
            .unwrap();
        } else {
            writeln!(out, "{:02}:{:02}", (a + b) / 60, (a + b) % 60).unwrap();
        }
    }
}
