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

    let n = scan.token::<i64>();
    let mut rings = BTreeMap::new();

    for _ in 0..n {
        let (name, ring) = (scan.token::<String>(), scan.token::<String>());

        if ring == "-" {
            continue;
        }

        rings.entry(ring).or_insert(Vec::new()).push(name);
    }

    let mut ret = Vec::new();

    for (_, names) in rings {
        if names.len() == 2 {
            ret.push((names[0].clone(), names[1].clone()));
        }
    }

    writeln!(out, "{}", ret.len()).unwrap();

    for (name1, name2) in ret {
        writeln!(out, "{name1} {name2}").unwrap();
    }
}
