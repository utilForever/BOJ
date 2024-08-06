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
    let mut clock_cur = 0;
    let mut direction = 1;

    for _ in 0..n {
        let (s, x) = (scan.token::<String>(), scan.token::<i64>());
        let is_synchronous = (clock_cur + 1) == x && s != "HOURGLASS";

        writeln!(
            out,
            "{} {}",
            clock_cur + 1,
            if is_synchronous { "YES" } else { "NO" }
        )
        .unwrap();

        if (clock_cur + 1) != x && s == "HOURGLASS" {
            direction = -direction;
        }

        clock_cur = if direction == 1 {
            (clock_cur + 1) % 12
        } else {
            (clock_cur + 11) % 12
        };
    }
}
