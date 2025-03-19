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
        let mut dice_p1 = [0; 2];
        let mut dice_p2 = [0; 2];

        for i in 0..2 {
            dice_p1[i] = scan.token::<i64>();
        }

        for i in 0..2 {
            dice_p2[i] = scan.token::<i64>();
        }

        if dice_p1[0] == 0 && dice_p1[1] == 0 && dice_p2[0] == 0 && dice_p2[1] == 0 {
            break;
        }

        dice_p1.sort_by(|a, b| b.cmp(a));
        dice_p2.sort_by(|a, b| b.cmp(a));

        writeln!(
            out,
            "{}.",
            if dice_p1 == [2, 1] && dice_p2 == [2, 1] {
                "Tie"
            } else if dice_p1 == [2, 1] {
                "Player 1 wins"
            } else if dice_p2 == [2, 1] {
                "Player 2 wins"
            } else if dice_p1[0] == dice_p1[1] && dice_p2[0] == dice_p2[1] {
                if dice_p1[0] == dice_p2[0] {
                    "Tie"
                } else if dice_p1[0] > dice_p2[0] {
                    "Player 1 wins"
                } else {
                    "Player 2 wins"
                }
            } else if dice_p1[0] == dice_p1[1] {
                "Player 1 wins"
            } else if dice_p2[0] == dice_p2[1] {
                "Player 2 wins"
            } else if dice_p1[0] == dice_p2[0] {
                if dice_p1[1] == dice_p2[1] {
                    "Tie"
                } else if dice_p1[1] > dice_p2[1] {
                    "Player 1 wins"
                } else {
                    "Player 2 wins"
                }
            } else {
                if dice_p1[0] > dice_p2[0] {
                    "Player 1 wins"
                } else {
                    "Player 2 wins"
                }
            }
        )
        .unwrap();
    }
}
