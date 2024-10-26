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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut positions = [(0, 0); 9];
    let mut idx = 0;
    let mut ret = [0; 9];

    for _ in 0..r {
        let row = scan.token::<String>().chars().collect::<Vec<_>>();
        let pos = row.iter().rev().position(|&c| c.is_numeric());

        if pos.is_none() {
            continue;
        }

        let pos = pos.unwrap();
        let team = row[c - pos - 1].to_digit(10).unwrap() as usize - 1;

        positions[idx] = (team, pos);
        idx += 1;
    }

    positions.sort_by(|a, b| a.1.cmp(&b.1));

    let mut idx = 0;
    let mut rank = 1;

    while idx < 9 {
        ret[positions[idx].0] = rank;
        idx += 1;

        while idx < 9 {
            if positions[idx].1 == positions[idx - 1].1 {
                ret[positions[idx].0] = rank;
                idx += 1;
            } else {
                break;
            }
        }

        rank += 1;
    }

    for rank in ret {
        writeln!(out, "{rank}").unwrap();
    }
}
