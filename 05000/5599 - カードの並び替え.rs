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

    let n = scan.token::<i64>();
    let mut cards = (1..=2 * n).collect::<Vec<i64>>();

    let m = scan.token::<i64>();

    for _ in 0..m {
        let k = scan.token::<usize>();

        if k == 0 {
            let (left, right) = cards.split_at_mut(n as usize);
            cards = left
                .iter()
                .zip(right.iter())
                .flat_map(|(l, r)| vec![*l, *r])
                .collect();
        } else {
            let (left, right) = cards.split_at_mut(k);
            cards = right.iter().chain(left.iter()).copied().collect();
        }
    }

    for card in cards {
        writeln!(out, "{card}").unwrap();
    }
}
