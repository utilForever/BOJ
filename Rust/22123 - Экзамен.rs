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

    for _ in 0..n {
        let (s, f, k) = (
            scan.token::<String>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );
        let s = s
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let f = f
            .split(':')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let s = s[0] * 3600 + s[1] * 60 + s[2];
        let f = f[0] * 3600 + f[1] * 60 + f[2];
        let mut diff = f - s;

        if diff <= 0 {
            diff += 24 * 3600;
        }

        writeln!(
            out,
            "{}",
            if diff >= k * 60 {
                "Perfect"
            } else if diff + 3600 >= k * 60 {
                "Test"
            } else {
                "Fail"
            }
        )
        .unwrap();
    }
}
