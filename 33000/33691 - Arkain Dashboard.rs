use io::Write;
use std::{
    collections::{HashMap, HashSet},
    io, str,
};

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

    let n = scan.token::<i64>();
    let mut containers = HashMap::new();

    for i in 0..n {
        let name = scan.token::<String>();
        containers.insert(name.clone(), i);
    }

    let k = scan.token::<i64>();
    let mut containers_fixed = HashSet::new();

    for _ in 0..k {
        let name = scan.token::<String>();
        containers_fixed.insert(name.clone());
    }

    let mut entries = containers
        .into_iter()
        .map(|(name, order)| {
            let fixed = containers_fixed.contains(&name);
            (name, order, fixed)
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| match b.2.cmp(&a.2) {
        std::cmp::Ordering::Equal => b.1.cmp(&a.1),
        other => other,
    });

    for (name, _, _) in entries {
        writeln!(out, "{name}").unwrap();
    }
}
