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

    let n = scan.token::<usize>();
    let mut coins = vec![0; n];

    for i in 0..n {
        coins[i] = scan.token::<usize>();
    }

    let mut greedy = vec![0; 100001];
    let mut optimal = vec![0; 100001];

    for i in 1..=100_000 {
        optimal[i] = coins
            .iter()
            .filter(|&&coin| coin <= i)
            .map(|&coin| optimal[i - coin] + 1)
            .min()
            .unwrap();
        greedy[i] = greedy[i - coins.iter().filter(|&&coin| coin <= i).max().unwrap()] + 1;

        if optimal[i] != greedy[i] {
            writeln!(out, "{}", i).unwrap();
            return;
        }
    }

    writeln!(out, "-1").unwrap();
}
