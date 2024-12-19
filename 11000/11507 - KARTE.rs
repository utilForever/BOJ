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

    let s = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut cards = [[false; 13]; 4];
    let mut idx = 0;

    while idx < s.len() {
        let suit = match s[idx] {
            'P' => 0,
            'K' => 1,
            'H' => 2,
            'T' => 3,
            _ => unreachable!(),
        };
        let rank =
            (s[idx + 1].to_digit(10).unwrap() * 10 + s[idx + 2].to_digit(10).unwrap()) as usize - 1;

        if cards[suit][rank] {
            writeln!(out, "GRESKA").unwrap();
            return;
        }

        cards[suit][rank] = true;
        idx += 3;
    }

    writeln!(
        out,
        "{} {} {} {}",
        13 - cards[0].iter().filter(|&&x| x).count(),
        13 - cards[1].iter().filter(|&&x| x).count(),
        13 - cards[2].iter().filter(|&&x| x).count(),
        13 - cards[3].iter().filter(|&&x| x).count()
    )
    .unwrap();
}
