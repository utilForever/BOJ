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

    let p = scan.token::<i64>();

    for _ in 0..p {
        let s = scan.token::<String>();
        let mut coin_sequence = vec![0; 8];

        let s = s.chars().collect::<Vec<char>>();

        for coins in s.windows(3) {
            match (coins[0], coins[1], coins[2]) {
                ('T', 'T', 'T') => coin_sequence[0] += 1,
                ('T', 'T', 'H') => coin_sequence[1] += 1,
                ('T', 'H', 'T') => coin_sequence[2] += 1,
                ('T', 'H', 'H') => coin_sequence[3] += 1,
                ('H', 'T', 'T') => coin_sequence[4] += 1,
                ('H', 'T', 'H') => coin_sequence[5] += 1,
                ('H', 'H', 'T') => coin_sequence[6] += 1,
                ('H', 'H', 'H') => coin_sequence[7] += 1,
                _ => (),
            }
        }

        for coin in coin_sequence.iter() {
            write!(out, "{} ", coin).unwrap();
        }
        writeln!(out).unwrap();
    }
}
