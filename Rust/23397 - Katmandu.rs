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

    let (t, d, m) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<usize>(),
    );
    let mut times = vec![0; m];

    for i in 0..m {
        times[i] = scan.token::<i64>();
    }

    if t > d {
        writeln!(out, "N").unwrap();
        return;
    }

    if m == 0 {
        writeln!(out, "Y").unwrap();
        return;
    }

    let mut ret = false;

    if times[0] >= t {
        ret = true;
    }

    times.windows(2).for_each(|w| {
        if w[1] - w[0] >= t {
            ret = true;
        }
    });

    if d - times[m - 1] >= t {
        ret = true;
    }

    writeln!(out, "{}", if ret { "Y" } else { "N" }).unwrap();
}
