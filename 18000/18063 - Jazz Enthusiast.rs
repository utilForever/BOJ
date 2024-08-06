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

    let (n, c) = (scan.token::<i64>(), scan.token::<i64>());
    let mut time_total = 0;

    for _ in 0..n {
        let time = scan.token::<String>();
        let time = time.split(':').collect::<Vec<_>>();

        time_total += time[0].parse::<i64>().unwrap() * 60 + time[1].parse::<i64>().unwrap();
    }

    time_total -= c * (n - 1);

    writeln!(
        out,
        "{:02}:{:02}:{:02}",
        time_total / 3600,
        (time_total % 3600) / 60,
        time_total % 60
    )
    .unwrap();
}
