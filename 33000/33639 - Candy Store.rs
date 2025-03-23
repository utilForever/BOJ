use io::Write;
use std::{collections::HashMap, io, str};

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

    let (n, q) = (scan.token::<i64>(), scan.token::<i64>());
    let mut names: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for _ in 0..n {
        let (fisrt, last) = (scan.token::<String>(), scan.token::<String>());
        let first_char = fisrt.chars().next().unwrap();
        let last_char = last.chars().next().unwrap();
        let initial = String::from(format!("{first_char}{last_char}"));

        names
            .entry(initial)
            .or_insert(Vec::new())
            .push((fisrt, last));
    }

    for _ in 0..q {
        let initial = scan.token::<String>();

        if let Some(names) = names.get(&initial) {
            if names.len() == 1 {
                writeln!(out, "{} {}", names[0].0, names[0].1).unwrap();
            } else {
                writeln!(out, "ambiguous").unwrap();
            }
        } else {
            writeln!(out, "nobody").unwrap();
        }
    }
}
