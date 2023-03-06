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

    let mut card_a = [0; 10];
    let mut card_b = [0; 10];
    let mut point_a = 0;
    let mut point_b = 0;
    let mut win_a = 0;
    let mut win_b = 0;
    let mut win_last = ' ';

    for i in 0..10 {
        card_a[i] = scan.token::<i64>();
    }

    for i in 0..10 {
        card_b[i] = scan.token::<i64>();
    }

    for i in 0..10 {
        if card_a[i] > card_b[i] {
            point_a += 3;
            win_a += 1;
            win_last = 'A';
        } else if card_a[i] < card_b[i] {
            point_b += 3;
            win_b += 1;
            win_last = 'B';
        } else {
            point_a += 1;
            point_b += 1;
        }
    }

    writeln!(out, "{point_a} {point_b}").unwrap();
    writeln!(
        out,
        "{}",
        if win_a > win_b {
            "A"
        } else if win_a < win_b {
            "B"
        } else if point_a > point_b {
            "A"
        } else if point_a < point_b {
            "B"
        } else if win_last == 'A' {
            "A"
        } else if win_last == 'B' {
            "B"
        } else {
            "D"
        }
    )
    .unwrap();
}
