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

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<i64>();
        let mut names: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for _ in 0..n {
            let s = scan.line().to_string();
            let pos = s.find(|c: char| c.is_numeric()).unwrap();
            let (name, year) = s.split_at(pos);
            let mut name = name.to_string();

            name.pop();

            names
                .entry(name)
                .and_modify(|years| years.push(year.to_string()))
                .or_insert(vec![year.to_string()]);
        }

        for (_, years) in names.iter_mut() {
            years.sort();
            years.dedup();
        }

        writeln!(out, "Case #{i}:").unwrap();

        for (name, years) in names {
            if years.len() >= 5 {
                continue;
            }

            writeln!(out, "{name}").unwrap();
        }
    }
}
