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

    // S : 0, B : 1, V : 2, K : 3
    let mut cards = [false; 53];

    for _ in 0..51 {
        let (suit, rank) = (scan.token::<char>(), scan.token::<i64>());
        let idx = match suit {
            'S' => 0,
            'B' => 13,
            'V' => 26,
            'K' => 39,
            _ => unreachable!(),
        } + rank as usize
            - 1;

        cards[idx] = true;
    }

    let ret = cards.iter().position(|&x| !x).unwrap();

    writeln!(
        out,
        "{} {}",
        match ret + 1 {
            1..=13 => 'S',
            14..=26 => 'B',
            27..=39 => 'V',
            40..=52 => 'K',
            _ => unreachable!(),
        },
        match ret + 1 {
            1..=13 => ret,
            14..=26 => ret - 13,
            27..=39 => ret - 26,
            40..=52 => ret - 39,
            _ => unreachable!(),
        } + 1
    )
    .unwrap();
}
