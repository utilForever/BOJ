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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (kari_start, ola_start, kari_end, ola_end) = (
        (scan.token::<i64>(), scan.token::<i64>()),
        (scan.token::<i64>(), scan.token::<i64>()),
        (scan.token::<i64>(), scan.token::<i64>()),
        (scan.token::<i64>(), scan.token::<i64>()),
    );

    let dist_start = (kari_start.0 - ola_start.0).pow(2) + (kari_start.1 - ola_start.1).pow(2);
    let dist_end = (kari_end.0 - ola_end.0).pow(2) + (kari_end.1 - ola_end.1).pow(2);

    writeln!(
        out,
        "{:.9}",
        (dist_start as f64).sqrt().max((dist_end as f64).sqrt())
    )
    .unwrap();
}
