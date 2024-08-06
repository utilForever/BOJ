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

    let (n, c) = (scan.token::<usize>(), scan.token::<i64>());
    let mut coins = vec![(0, 0); n];

    for i in 0..n {
        coins[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    coins.sort_by(|a, b| b.0.cmp(&a.0));

    let mut ret = 0;

    for i in 0..n {
        if coins[i].0 >= c {
            ret += coins[i].1;
            coins[i].1 = 0;
        }
    }

    coins.retain(|&x| x.1 > 0);

    loop {
        let mut value = c;

        for i in 0..coins.len() {
            let mut cnt = value / coins[i].0;
            cnt = cnt.min(coins[i].1);

            value -= cnt * coins[i].0;
            coins[i].1 -= cnt;
        }

        coins.retain(|&x| x.1 > 0);

        if value > 0 {
            if coins.is_empty() {
                break;
            }

            coins.last_mut().unwrap().1 -= 1;
            value = 0;
        }

        if value > 0 {
            break;
        }

        coins.retain(|&x| x.1 > 0);

        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
