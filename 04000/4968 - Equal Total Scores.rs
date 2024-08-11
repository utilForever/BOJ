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

    loop {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());

        if n == 0 && m == 0 {
            break;
        }

        let mut cards_taro = vec![0; n];
        let mut cards_hanako = vec![0; m];

        for i in 0..n {
            cards_taro[i] = scan.token::<i64>();
        }

        for i in 0..m {
            cards_hanako[i] = scan.token::<i64>();
        }

        let sum_taro = cards_taro.iter().sum::<i64>();
        let sum_hanako = cards_hanako.iter().sum::<i64>();
        let diff_sum = sum_taro - sum_hanako;

        let mut idx_taro = 0;
        let mut idx_hanako = 0;
        let mut sum_cards = i64::MAX;

        for i in 0..n {
            for j in 0..m {
                if (cards_hanako[j] - cards_taro[i]) * -2 == diff_sum
                    && cards_hanako[j] + cards_taro[i] < sum_cards
                {
                    idx_taro = i;
                    idx_hanako = j;
                    sum_cards = cards_hanako[j] + cards_taro[i];
                }
            }
        }

        if sum_cards == i64::MAX {
            writeln!(out, "-1").unwrap();
        } else {
            writeln!(out, "{} {}", cards_taro[idx_taro], cards_hanako[idx_hanako]).unwrap();
        }
    }
}
