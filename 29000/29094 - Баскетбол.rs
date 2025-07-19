use io::Write;
use std::{collections::HashMap, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let n = scan.token::<i64>();
    let mut players = HashMap::new();

    for _ in 0..n {
        players.insert(scan.token::<String>(), 0);
    }

    let m = scan.token::<i64>();
    let mut score_a = 0;
    let mut score_b = 0;

    for _ in 0..m {
        let (score, player) = (scan.token::<String>(), scan.token::<String>());
        let score = score.split(":").collect::<Vec<&str>>();
        let diff_a = score[0].parse::<i64>().unwrap() - score_a;
        let diff_b = score[1].parse::<i64>().unwrap() - score_b;

        players.entry(player).and_modify(|s| {
            *s += diff_a;
            *s += diff_b;
        });

        score_a += diff_a;
        score_b += diff_b;
    }

    let max = *players.values().max().unwrap();
    let player = players
        .iter()
        .find(|&(_, &v)| v == max)
        .map(|(k, _)| k)
        .unwrap();

    writeln!(out, "{player} {max}").unwrap();
}
