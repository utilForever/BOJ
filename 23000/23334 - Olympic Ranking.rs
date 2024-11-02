use io::Write;
use std::{io, str};

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

    let n = scan.token::<usize>();
    let mut countries = vec![(0, 0, 0, String::new()); n];

    for _ in 0..n {
        let line = scan
            .line()
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        let (gold, silver, bronze) = (
            line[0].parse::<i64>().unwrap(),
            line[1].parse::<i64>().unwrap(),
            line[2].parse::<i64>().unwrap(),
        );
        let mut name = String::new();

        for i in 3..line.len() {
            name.push_str(&line[i]);

            if i != line.len() - 1 {
                name.push(' ');
            }
        }

        countries.push((gold, silver, bronze, name));
    }

    countries.sort_by(|a, b| {
        if a.0 != b.0 {
            return b.0.cmp(&a.0);
        } else if a.1 != b.1 {
            return b.1.cmp(&a.1);
        } else if a.2 != b.2 {
            return b.2.cmp(&a.2);
        } else {
            return a.3.cmp(&b.3);
        }
    });

    writeln!(out, "{}", countries[0].3).unwrap();
}
