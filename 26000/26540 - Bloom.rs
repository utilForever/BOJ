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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let x = scan.token::<usize>();
        let mut plants = vec![Vec::new(); x];

        for i in 0..x {
            let stages = scan.line().trim().to_string();
            let stages = stages
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            plants[i] = stages;
        }

        let day = scan.token::<i64>();
        let mut ret = 0;

        for plant in plants {
            if *plant.last().unwrap() == -1 {
                let sum = plant[..plant.len() - 1].iter().sum::<i64>();

                if sum == day {
                    ret += 1;
                }
            } else {
                let idx = *plant.last().unwrap() as usize;
                let remain = day - plant[..idx].iter().sum::<i64>();

                if remain > 0 && remain % plant[idx..plant.len() - 1].iter().sum::<i64>() == 0 {
                    ret += 1;
                }
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
