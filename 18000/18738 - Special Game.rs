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

enum Turn {
    Dmytryk,
    Petro,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut cards_dmytryk = vec![0; n];
    let mut cards_petro = vec![0; n];

    for i in 0..n {
        cards_dmytryk[i] = scan.token::<usize>();
    }

    for i in 0..n {
        cards_petro[i] = scan.token::<usize>();
    }

    cards_dmytryk.sort();
    cards_petro.sort();

    let mut turn = Turn::Dmytryk;
    let mut ret = 0;

    loop {
        match turn {
            Turn::Dmytryk => {
                if cards_dmytryk.is_empty() || *cards_dmytryk.last().unwrap() < cards_petro[0] {
                    break;
                }

                if cards_dmytryk[0] > *cards_petro.last().unwrap() {
                    ret += cards_dmytryk.len() as i64;
                    break;
                }

                let mut candidates = Vec::new();
                let num = cards_dmytryk.len() as i64;
                let mut best_dmytryk = -num;
                let mut best_petro = num;

                for i in 0..cards_dmytryk.len() as i64 {
                    let mut j = 0;

                    while j < num && cards_dmytryk[i as usize] >= cards_petro[j as usize] {
                        j += 1;
                    }

                    if j < num {
                        candidates.push((i, j));
                    }
                }

                for (i, j) in candidates {
                    if i - j > best_dmytryk - best_petro {
                        best_dmytryk = i;
                        best_petro = j;
                    }
                }

                cards_dmytryk.remove(best_dmytryk as usize);
                cards_petro.remove(best_petro as usize);

                ret += num - 1;
                turn = Turn::Petro;
            }
            Turn::Petro => {
                if cards_petro.is_empty() || *cards_petro.last().unwrap() < cards_dmytryk[0] {
                    break;
                }

                if cards_petro[0] > *cards_dmytryk.last().unwrap() {
                    ret -= cards_petro.len() as i64;
                    break;
                }

                let mut candidates = Vec::new();
                let num = cards_petro.len() as i64;
                let mut best_dmytryk = num;
                let mut best_petro = -num;

                for i in 0..cards_petro.len() as i64 {
                    let mut j = 0;

                    while j < num && cards_petro[i as usize] >= cards_dmytryk[j as usize] {
                        j += 1;
                    }

                    if j < num {
                        candidates.push((i, j));
                    }
                }

                for (i, j) in candidates {
                    if i - j > best_petro - best_dmytryk {
                        best_petro = i;
                        best_dmytryk = j;
                    }
                }

                cards_dmytryk.remove(best_dmytryk as usize);
                cards_petro.remove(best_petro as usize);

                ret -= num - 1;
                turn = Turn::Dmytryk;
            }
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
