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

    let n = scan.token::<i64>();

    for _ in 0..n {
        let line = scan.line().trim().to_string();
        let data = line.split_whitespace().collect::<Vec<&str>>();
        let mut nums = Vec::new();
        let mut operators = Vec::new();

        for datum in data {
            if datum.parse::<i64>().is_ok() {
                nums.push(datum.parse::<i64>().unwrap());
            } else {
                operators.push(datum);
            }
        }

        // First, process *, / and %
        let mut idx = 0;

        while idx < operators.len() {
            if operators[idx] == "*" || operators[idx] == "/" || operators[idx] == "%" {
                let left = nums[idx];
                let right = nums[idx + 1];
                let operator = operators[idx];

                let result = match operator {
                    "*" => left * right,
                    "/" => left / right,
                    "%" => left % right,
                    _ => unreachable!(),
                };

                nums[idx] = result;
                nums.remove(idx + 1);
                operators.remove(idx);
            } else {
                idx += 1;
            }
        }

        // Then, process + and -
        let mut idx = 0;

        while idx < operators.len() {
            if operators[idx] == "+" || operators[idx] == "-" {
                let left = nums[idx];
                let right = nums[idx + 1];
                let operator = operators[idx];

                let result = match operator {
                    "+" => left + right,
                    "-" => left - right,
                    _ => unreachable!(),
                };

                nums[idx] = result;
                nums.remove(idx + 1);
                operators.remove(idx);
            } else {
                idx += 1;
            }
        }

        writeln!(out, "{}", nums[0]).unwrap();
    }
}
