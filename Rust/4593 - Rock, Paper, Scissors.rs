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

    loop {
        let (symbols1, symbols2) = (scan.token::<String>(), scan.token::<String>());

        if symbols1 == "E" && symbols2 == "E" {
            break;
        }

        let mut win_player1 = 0;
        let mut win_player2 = 0;

        let symbols1 = symbols1.chars().collect::<Vec<_>>();
        let symbols2 = symbols2.chars().collect::<Vec<_>>();

        for (symbol1, symbol2) in symbols1.iter().zip(symbols2.iter()) {
            if symbol1 == symbol2 {
                continue;
            }

            if (symbol1 == &'R' && symbol2 == &'S')
                || (symbol1 == &'S' && symbol2 == &'P')
                || (symbol1 == &'P' && symbol2 == &'R')
            {
                win_player1 += 1;
            } else {
                win_player2 += 1;
            }
        }

        writeln!(out, "P1: {win_player1}").unwrap();
        writeln!(out, "P2: {win_player2}").unwrap();
    }
}
