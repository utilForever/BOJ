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

    let (n, m) = (scan.token::<i64>(), scan.token::<i64>());
    let mut diseases = HashSet::new();

    let k = scan.token::<i64>();

    for _ in 0..k {
        diseases.insert(scan.token::<i64>());
    }

    let mut symptoms = HashSet::new();

    for i in 1..=n {
        let p = scan.token::<i64>();

        if !diseases.contains(&i) {
            for _ in 0..p {
                let _ = scan.token::<i64>();
            }
        } else {
            for _ in 0..p {
                symptoms.insert(scan.token::<i64>());
            }
        }
    }

    let mut symptoms = symptoms.into_iter().collect::<Vec<_>>();

    symptoms.sort_unstable();
    symptoms.dedup();

    writeln!(
        out,
        "{}",
        if symptoms == (1..=m).collect::<Vec<_>>() {
            "yes"
        } else {
            "no"
        }
    )
    .unwrap();
}
