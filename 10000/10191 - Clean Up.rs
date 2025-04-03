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

    let mut rosters = vec![(String::new(), 0, 0); 9];

    for i in 0..9 {
        rosters[i] = (
            scan.token::<String>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    for i in 0..9 {
        let (hits, bats) = (scan.token::<i64>(), scan.token::<i64>());
        rosters[i].1 += hits;
        rosters[i].2 += bats;
    }

    let avg = rosters
        .iter()
        .map(|x| x.1 as f64 / x.2 as f64)
        .collect::<Vec<f64>>();
    let (idx, _) = avg
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .unwrap();

    rosters.swap(idx, 3);

    for player in rosters {
        writeln!(out, "{}", player.0).unwrap();
    }
}
