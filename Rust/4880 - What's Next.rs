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
        let (a1, a2, a3) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );

        if a1 == 0 && a2 == 0 && a3 == 0 {
            break;
        }

        writeln!(
            out,
            "{} {}",
            if a2 - a1 == a3 - a2 { "AP" } else { "GP" },
            if a2 - a1 == a3 - a2 {
                a3 + (a2 - a1)
            } else {
                a3 * (a2 / a1)
            }
        )
        .unwrap();
    }
}
