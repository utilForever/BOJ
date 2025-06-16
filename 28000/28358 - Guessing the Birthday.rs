use io::Write;
use std::{io, str, vec};

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

    let days = [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut dates = Vec::with_capacity(366);

    for i in 1..=12 {
        for j in 1..=days[i] {
            dates.push(format!("{i}{j}").to_string());
        }
    }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut ret = dates.clone();

        for i in 0..10 {
            let check = scan.token::<i64>();

            if check == 1 {
                ret = ret
                    .into_iter()
                    .filter(|date| !date.contains(&i.to_string()))
                    .collect();
            }
        }

        writeln!(out, "{}", ret.len()).unwrap();
    }
}
