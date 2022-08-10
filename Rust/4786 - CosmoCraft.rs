use io::Write;
use std::{cmp, io, str};

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

// Reference: https://programmingteam.cc.gatech.edu/contest/UC2012.pdf
// Thanks for @pentagon03 to explain the editorial!
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let (n, k, t) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
        );

        // End condition
        if n == 0 && k == 0 && t == 0 {
            break;
        }

        let mut attacks = vec![0; t + 1];

        for i in 1..t {
            attacks[i] = scan.token::<i64>();
        }

        let mut income = n;
        let mut production = k;
        let mut army_player = 0;
        let mut army_opponent = attacks[1];

        // Observations
        // 1) The maximum army unit the player can produce in each turn is min(income, production)
        // 2) The maximum army unit that can produce in the last two turns is always min(income, production) + production
        // 3) Producing army unit in advance is only useful for the next turn.
        //    If the player passes more than two turns, it's better to make it on the last turn
        // Consider three cases
        // 1) Cannot survive until the next turn
        // 2) Need to produce army unit(s) at the next turn
        // 3) Don't need to product army unit(s) at the next turn
        for i in 1..=t {
            if i + 2 > t {
                army_player += cmp::min(income, production) - army_opponent;
                production = income;
                army_opponent = 0;

                if i == t {
                    writeln!(out, "{army_player}").unwrap();
                }
            } else if attacks[i + 1] > cmp::min(income, production) + income - army_player {
                writeln!(out, "-1").unwrap();
                break;
            } else if attacks[i + 1] <= income {
                let diff = cmp::min(income, production) - army_opponent;

                production += income - army_opponent - diff;
                income += diff;
                army_opponent = attacks[i + 1];
            } else {
                let diff = attacks[i + 1] - income;
                let diff_income = cmp::min(income, production) - army_opponent - diff;
                let diff_army = cmp::min(cmp::max(0, production - income), diff);

                production += income - army_opponent - diff - diff_income;
                army_opponent = income + diff_army;
                income += diff_income + diff_army;
            }
        }
    }
}
