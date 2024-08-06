use io::Write;
use std::{collections::HashSet, io, str};

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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut sets = vec![HashSet::new(); n];

    for i in 0..n {
        let len = scan.token::<usize>();

        for _ in 0..len {
            sets[i].insert(scan.token::<i64>());
        }
    }

    for _ in 0..q {
        let command = scan.token::<i64>();

        if command == 1 {
            let (a, b) = (scan.token::<usize>() - 1, scan.token::<usize>() - 1);

            let (set_a, set_b) = {
                if a < b {
                    let (set_a, set_b) = sets.split_at_mut(b);
                    (&mut set_a[a], &mut set_b[0])
                } else {
                    let (set_a, set_b) = sets.split_at_mut(a);
                    (&mut set_b[0], &mut set_a[b])
                }
            };

            if set_a.len() < set_b.len() {
                set_b.extend(set_a.iter().copied());
                std::mem::swap(set_a, set_b);
            } else {
                set_a.extend(set_b.iter().copied());
            }

            *set_b = HashSet::new();
        } else {
            let a = scan.token::<usize>() - 1;
            writeln!(out, "{}", sets[a].len()).unwrap();
        }
    }
}
