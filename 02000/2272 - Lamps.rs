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

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut lamps = vec![0; n];

    for i in 0..n {
        lamps[i] = scan.token::<i64>();
    }

    let mut m_log = 0;
    let mut idx = 1;

    for i in (0..=62).rev() {
        if (m >> i) & 1 == 1 {
            m_log = i;
            break;
        }
    }

    for i in 0..=m_log {
        let mut lamps_new = vec![0; n];

        if (m >> i) & 1 == 1 {
            for j in 0..n {
                lamps_new[j] = lamps[j] ^ lamps[(j + idx) % n];
            }

            lamps = lamps_new;
        }

        idx = (idx * 2) % n;
    }

    for val in lamps {
        writeln!(out, "{val}").unwrap();
    }
}
