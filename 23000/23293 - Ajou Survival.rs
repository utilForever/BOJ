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

#[derive(Clone)]
struct Player {
    area: usize,
    items: Vec<i64>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            area: 1,
            items: vec![0; 54],
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (t, n) = (scan.token::<i64>(), scan.token::<usize>());
    let mut players = vec![Player::default(); n + 1];
    let mut logs_invalid = Vec::new();
    let mut players_block = Vec::new();

    for _ in 0..t {
        let (num, player, code, factor) = (
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<char>(),
            scan.token::<usize>(),
        );

        match code {
            'M' => {
                players[player].area = factor;
            }
            'F' => {
                if players[player].area != factor {
                    logs_invalid.push(num);
                }

                players[player].items[factor] += 1;
            }
            'C' => {
                let factor2 = scan.token::<usize>();

                if players[player].items[factor] == 0 || players[player].items[factor2] == 0 {
                    logs_invalid.push(num);
                }

                players[player].items[factor] -= 1;
                players[player].items[factor2] -= 1;

                if players[player].items[factor] < 0 {
                    players[player].items[factor] = 0;
                }
                if players[player].items[factor2] < 0 {
                    players[player].items[factor2] = 0;
                }
            }
            'A' => {
                if players[player].area != players[factor].area {
                    logs_invalid.push(num);
                    players_block.push(player);
                }
            }
            _ => unreachable!(),
        }
    }

    players_block.sort();
    players_block.dedup();

    writeln!(out, "{}", logs_invalid.len()).unwrap();

    if !logs_invalid.is_empty() {
        for log in logs_invalid {
            write!(out, "{log} ").unwrap();
        }

        writeln!(out).unwrap();
    }

    writeln!(out, "{}", players_block.len()).unwrap();

    if !players_block.is_empty() {
        for player in players_block {
            write!(out, "{player} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
