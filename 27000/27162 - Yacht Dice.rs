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

    let s = scan.token::<String>();
    let s = s.chars().collect::<Vec<_>>();
    let mut scores = [0; 12];
    let mut dices = [0; 3];

    for i in 0..3 {
        dices[i] = scan.token::<i64>();
    }

    dices.sort();

    // Ones
    if s[0] == 'Y' {
        scores[0] = dices.iter().filter(|&x| *x == 1).count() as i64 + 2;
    }

    // Twos
    if s[1] == 'Y' {
        scores[1] = dices.iter().filter(|&x| *x == 2).count() as i64 * 2 + 4;
    }

    // Threes
    if s[2] == 'Y' {
        scores[2] = dices.iter().filter(|&x| *x == 3).count() as i64 * 3 + 6;
    }

    // Fours
    if s[3] == 'Y' {
        scores[3] = dices.iter().filter(|&x| *x == 4).count() as i64 * 4 + 8;
    }

    // Fives
    if s[4] == 'Y' {
        scores[4] = dices.iter().filter(|&x| *x == 5).count() as i64 * 5 + 10;
    }

    // Sixes
    if s[5] == 'Y' {
        scores[5] = dices.iter().filter(|&x| *x == 6).count() as i64 * 6 + 12;
    }

    // Four of a kind
    if s[6] == 'Y' {
        if dices[0] == dices[1] || dices[0] == dices[2] {
            scores[6] = dices[0] * 4;
        } else if dices[1] == dices[2] {
            scores[6] = dices[1] * 4;
        }
    }

    // Full house
    if s[7] == 'Y' {
        scores[7] = if dices[0] == dices[1] {
            if dices[1] == dices[2] {
                dices[0] * 3 + if dices[0] == 6 { 10 } else { 12 }
            } else {
                if dices[0] > dices[2] {
                    dices[0] * 3 + dices[2] * 2
                } else {
                    dices[0] * 2 + dices[2] * 3
                }
            }
        } else if dices[1] == dices[2] {
            if dices[0] > dices[1] {
                dices[0] * 3 + dices[1] * 2
            } else {
                dices[0] * 2 + dices[1] * 3
            }
        } else {
            0
        }
    }

    // Little straight
    if s[8] == 'Y'
        && (dices[0] != dices[1]
            && dices[0] != dices[2]
            && dices[1] != dices[2]
            && dices[0] >= 1
            && dices[0] <= 5
            && dices[1] >= 1
            && dices[1] <= 5
            && dices[2] >= 1
            && dices[2] <= 5)
    {
        scores[8] = 30;
    }

    // Big straight
    if s[9] == 'Y'
        && (dices[0] != dices[1]
            && dices[0] != dices[2]
            && dices[1] != dices[2]
            && dices[0] >= 2
            && dices[0] <= 6
            && dices[1] >= 2
            && dices[1] <= 6
            && dices[2] >= 2
            && dices[2] <= 6)
    {
        scores[9] = 30;
    }

    // Yacht
    if s[10] == 'Y' && dices[0] == dices[1] && dices[1] == dices[2] {
        scores[10] = 50;
    }

    // Choice
    if s[11] == 'Y' {
        scores[11] = dices.iter().sum::<i64>() + 12;
    }

    writeln!(out, "{}", scores.iter().max().unwrap()).unwrap();
}
