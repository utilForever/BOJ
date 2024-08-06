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

    let card_num = scan.token::<String>();
    let card_num = card_num.chars().collect::<Vec<char>>();
    let mut card_nums = card_num
        .iter()
        .map(|x| x.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();

    for i in (0..card_nums.len() - 1).rev().step_by(2) {
        card_nums[i] *= 2;
    }

    for num in card_nums.iter_mut() {
        if *num >= 10 {
            *num = *num / 10 + *num % 10;
        }
    }

    let ret = card_nums.iter().sum::<i64>();

    writeln!(out, "{}", if ret % 10 == 0 { "DA" } else { "NE" }).unwrap();
}
