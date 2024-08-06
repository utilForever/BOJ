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

    let n = scan.token::<usize>();
    let lyric = [
        "baby", "sukhwan", "tururu", "turu", "very", "cute", "tururu", "turu", "in", "bed",
        "tururu", "turu", "baby", "sukhwan",
    ];

    let (q, r) = ((n - 1) / 14, (n - 1) % 14);

    if r == 2 || r == 6 || r == 10 {
        if q >= 3 {
            writeln!(out, "tu+ru*{}", q + 2).unwrap();
        } else {
            write!(out, "{}", lyric[r]).unwrap();

            for _ in 0..q {
                write!(out, "ru").unwrap();
            }

            writeln!(out).unwrap();
        }
    } else if r == 3 || r == 7 || r == 11 {
        if q >= 4 {
            writeln!(out, "tu+ru*{}", q + 1).unwrap();
        } else {
            write!(out, "{}", lyric[r]).unwrap();

            for _ in 0..q {
                write!(out, "ru").unwrap();
            }

            writeln!(out).unwrap();
        }
    } else {
        writeln!(out, "{}", lyric[r]).unwrap();
    }
}
