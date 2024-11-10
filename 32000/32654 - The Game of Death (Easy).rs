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

    let n = scan.token::<usize>();
    let mut graph = vec![Vec::new(); n + 1];

    for i in 1..=n {
        let (l, r) = (scan.token::<usize>(), scan.token::<usize>());
        graph[i].push(l);
        graph[i].push(r);
    }

    let mut cycles = HashSet::new();
    let mut positions = HashSet::new();

    positions.insert(1);

    for length in 1..=100 {
        let mut positions_next = HashSet::new();

        for &curr in positions.iter() {
            for &next in graph[curr].iter() {
                positions_next.insert(next);
            }
        }

        if positions_next.contains(&1) {
            cycles.insert(length);
        }

        std::mem::swap(&mut positions, &mut positions_next);
    }

    if cycles.is_empty() {
        writeln!(out, "10").unwrap();
        return;
    }

    let mut cycles = cycles.into_iter().collect::<Vec<_>>();
    cycles.sort();

    for k in 10..=99 {
        if cycles.contains(&k) {
            continue;
        }

        writeln!(out, "{k}").unwrap();
        return;
    }

    writeln!(out, "-1").unwrap();
}
