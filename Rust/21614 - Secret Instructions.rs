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

    let mut direction_prev = 0;

    loop {
        let n = scan.token::<i64>();

        if n == 99999 {
            break;
        }

        let direction = (n / 10000) + ((n / 1000) % 10);

        if direction == 0 {
            writeln!(
                out,
                "{} {}",
                if direction_prev == 1 { "left" } else { "right" },
                n % 1000
            )
            .unwrap();
        } else if direction % 2 == 1 {
            writeln!(out, "left {}", n % 1000).unwrap();
        } else {
            writeln!(out, "right {}", n % 1000).unwrap();
        }

        direction_prev = direction % 2;
    }
}
