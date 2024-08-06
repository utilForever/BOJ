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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut abilities = vec![(0, 0); n + 1];

    for i in 1..=n {
        abilities[i].0 = scan.token::<usize>();
        abilities[i].1 = i - 1;
    }

    abilities.sort();

    let mut min_diff = vec![0; n + 1];
    let mut missing_indexes = vec![Vec::new(); n + 1];

    missing_indexes[1].push(abilities[1].1);
    missing_indexes[2].push(abilities[1].1);
    missing_indexes[2].push(abilities[2].1);
    missing_indexes[3].push(abilities[1].1);
    missing_indexes[3].push(abilities[2].1);
    missing_indexes[3].push(abilities[3].1);

    for i in 4..=n {
        let diff = abilities[i].0 - abilities[i - 3].0;

        if i % 4 > 0 {
            min_diff[i] = cmp::min(min_diff[i - 1], min_diff[i - 4] + diff);

            if min_diff[i] == min_diff[i - 1] {
                for j in 0..missing_indexes[i - 1].len() {
                    let index = missing_indexes[i - 1][j];
                    missing_indexes[i].push(index);
                }

                missing_indexes[i].push(abilities[i].1);
            } else {
                for j in 0..missing_indexes[i - 4].len() {
                    let index = missing_indexes[i - 4][j];
                    missing_indexes[i].push(index);
                }
            }
        } else {
            min_diff[i] = min_diff[i - 4] + diff;
        }
    }

    writeln!(out, "{}", min_diff[n]).unwrap();

    if n % 4 > 0 {
        for i in 0..missing_indexes[n].len() {
            writeln!(out, "{}", missing_indexes[n][i]).unwrap();
        }
    }
}
