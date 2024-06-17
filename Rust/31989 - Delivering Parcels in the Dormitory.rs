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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut rooms = vec![0; 2 * n + 1];
    let mut dests = vec![0; m];
    let mut weights = vec![0; m];

    for i in 0..2 * n + 1 {
        rooms[i] = scan.token::<i64>();
    }

    for i in 0..m {
        dests[i] = scan.token::<usize>() - 1;
        weights[i] = scan.token::<i64>();
    }

    let dest_min = *dests.iter().min().unwrap();
    let dest_max = *dests.iter().max().unwrap();
    let mut ret = 0;

    if dest_min > n {
        for i in 0..m {
            ret += weights[i] * (rooms[dests[i]] - rooms[n]);
        }

        ret += 2 * (rooms[dest_max] - rooms[n]);
    } else if dest_max < n {
        for i in 0..m {
            ret += weights[i] * (rooms[n] - rooms[dests[i]]);
        }

        ret += 2 * (rooms[n] - rooms[dest_min]);
    } else {
        for i in 0..m {
            ret += if rooms[dests[i]] > rooms[n] {
                weights[i] * (rooms[dests[i]] - rooms[n])
            } else {
                weights[i] * (rooms[n] - rooms[dests[i]])
            };
        }

        ret += 2 * (rooms[dest_max] - rooms[dest_min]);
    }

    writeln!(out, "{ret}").unwrap();
}
