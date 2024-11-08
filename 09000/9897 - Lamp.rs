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

    let (l, g, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut lamps = vec![false; l + 1];
    let mut guards = vec![(String::new(), 0, 0); g];

    for i in 0..g {
        guards[i] = (
            scan.token::<String>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
    }

    for _ in 0..r {
        let guard = scan.token::<String>();
        let pos = guards.iter().position(|x| x.0 == guard).unwrap();
        let (_, mut a, d) = guards[pos];

        while a <= l {
            lamps[a] ^= true;
            a += d;
        }
    }

    writeln!(out, "{}", lamps.iter().filter(|&&x| x).count()).unwrap();
}
