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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<i64>(), scan.token::<usize>());
    let mut map = BTreeMap::new();

    for _ in 0..n {
        let word = scan.token::<String>();

        if word.len() < m {
            continue;
        }

        map.entry(word).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut words = map.into_iter().collect::<Vec<_>>();
    words.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then(b.0.len().cmp(&a.0.len()))
            .then(a.0.cmp(&b.0))
    });

    for (word, _) in words {
        writeln!(out, "{word}").unwrap();
    }
}
