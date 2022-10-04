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

    let prize_first = [5000000, 3000000, 2000000, 500000, 300000, 100000];
    let prize_second = [5120000, 2560000, 1280000, 640000, 320000];

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (a, b) = (scan.token::<i64>(), scan.token::<i64>());

        let prize_a = match a {
            1 => prize_first[0],
            2..=3 => prize_first[1],
            4..=6 => prize_first[2],
            7..=10 => prize_first[3],
            11..=15 => prize_first[4],
            16..=21 => prize_first[5],
            _ => 0,
        };
        let prize_b = match b {
            1 => prize_second[0],
            2..=3 => prize_second[1],
            4..=7 => prize_second[2],
            8..=15 => prize_second[3],
            16..=31 => prize_second[4],
            _ => 0,
        };

        writeln!(out, "{}", prize_a + prize_b).unwrap();
    }
}
