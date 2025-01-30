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

    let (t, _) = (scan.token::<usize>(), scan.token::<usize>());
    let mut results = vec![(0, 0); t];

    for i in 0..t {
        let result = scan.line().trim().to_string();
        let result = result.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut cnt_problem = 0;
        let mut time = 0;

        for val in result {
            if val == "X" {
                continue;
            }

            cnt_problem += 1;
            time += val.parse::<i64>().unwrap();
        }

        results[i] = (cnt_problem, time);
    }

    let mut ret = 0;

    for i in 1..t {
        if results[i].0 > results[0].0
            || (results[i].0 == results[0].0 && results[i].1 <= results[0].1)
        {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
