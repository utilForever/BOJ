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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cards_youngwoo = vec![0; n];
    let mut cards_minwoo = vec![0; n];
    let mut cards_woojin = vec![0; n];

    for i in 0..n {
        cards_youngwoo[i] = scan.token::<i64>();
    }

    for i in 0..n {
        cards_minwoo[i] = scan.token::<i64>();
    }

    for i in 0..n {
        cards_woojin[i] = scan.token::<i64>();
    }

    let woojin_min = *cards_woojin.iter().min().unwrap();
    let woojin_max = *cards_woojin.iter().max().unwrap();
    let val1 = 2 * k - (woojin_min + woojin_max);

    cards_youngwoo.sort_unstable();
    cards_minwoo.sort_unstable();

    let mut left = 0;
    let mut right = n;
    let mut ret = i64::MAX;

    while left < n && right > 0 {
        let val2 = 2 * (cards_youngwoo[left] + cards_minwoo[right - 1]);
        let diff = (val1 - val2).abs();

        ret = ret.min(diff);

        if ret == 0 {
            break;
        }

        if val1 > val2 {
            left += 1;
        } else {
            right -= 1;
        }
    }

    writeln!(out, "{}", (ret + (woojin_max - woojin_min)) / 2).unwrap();
}
