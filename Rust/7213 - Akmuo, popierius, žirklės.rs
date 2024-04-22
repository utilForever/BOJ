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

    // Scissor, Rock, Paper
    let (a1, p1, z1) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let (a2, p2, z2) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );

    let ret_max = {
        let (mut a1, mut p1, mut z1) = (a1, p1, z1);
        let (mut a2, mut p2, mut z2) = (a2, p2, z2);
        let mut ret = 0;

        // Count the number of wins
        let cnt = a1.min(z2);
        ret += cnt;
        a1 -= cnt;
        z2 -= cnt;

        let cnt = p1.min(a2);
        ret += cnt;
        p1 -= cnt;
        a2 -= cnt;

        let cnt = z1.min(p2);
        ret += cnt;
        z1 -= cnt;
        p2 -= cnt;

        // Count the number of losses
        let cnt = a1.min(p2);
        ret -= cnt;

        let cnt = p1.min(z2);
        ret -= cnt;

        let cnt = z1.min(a2);
        ret -= cnt;

        ret
    };

    let ret_min = {
        let (mut a1, mut p1, mut z1) = (a1, p1, z1);
        let (mut a2, mut p2, mut z2) = (a2, p2, z2);
        let mut ret = 0;

        // Count the number of losses
        let cnt = a1.min(p2);
        ret -= cnt;
        a1 -= cnt;
        p2 -= cnt;

        let cnt = p1.min(z2);
        ret -= cnt;
        p1 -= cnt;
        z2 -= cnt;

        let cnt = z1.min(a2);
        ret -= cnt;
        z1 -= cnt;
        a2 -= cnt;

        // Count the number of wins
        let cnt = a1.min(z2);
        ret += cnt;

        let cnt = p1.min(a2);
        ret += cnt;

        let cnt = z1.min(p2);
        ret += cnt;

        ret
    };

    writeln!(out, "{ret_max}").unwrap();
    writeln!(out, "{ret_min}").unwrap();
}
