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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();
        let mut win_player1 = 0;
        let mut win_player2 = 0;

        for _ in 0..n {
            let (p1, p2) = (scan.token::<String>(), scan.token::<String>());

            if (p1 == "R" && p2 == "S") || (p1 == "S" && p2 == "P") || (p1 == "P" && p2 == "R") {
                win_player1 += 1;
            } else if (p2 == "R" && p1 == "S")
                || (p2 == "S" && p1 == "P")
                || (p2 == "P" && p1 == "R")
            {
                win_player2 += 1;
            }
        }

        writeln!(
            out,
            "{}",
            if win_player1 > win_player2 {
                "Player 1"
            } else if win_player2 > win_player1 {
                "Player 2"
            } else {
                "TIE"
            }
        )
        .unwrap();
    }
}
