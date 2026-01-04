use io::Write;
use std::{collections::BTreeMap, io, str};

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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut baits = vec![0; n];
    let mut fishes = vec![0; m];

    for i in 0..n {
        baits[i] = scan.token::<i64>();
    }

    for i in 0..m {
        fishes[i] = scan.token::<i64>();
    }

    let mut map_baits = BTreeMap::new();

    for bait in baits {
        *map_baits.entry(bait).or_insert(0) += 1;
    }

    fishes.sort_unstable_by(|a, b| b.cmp(a));

    let mut ret = 0;

    for fish in fishes {
        if let Some((&bait, _)) = map_baits.range(..fish).next_back() {
            ret += fish;

            let cnt = map_baits.get_mut(&bait).unwrap();
            *cnt -= 1;

            if *cnt == 0 {
                map_baits.remove(&bait);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
