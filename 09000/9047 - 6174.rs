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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let mut num = scan.token::<String>().chars().collect::<Vec<_>>();
        let mut ret = 0;

        loop {
            if num == ['6', '1', '7', '4'] {
                break;
            }

            let mut num_min = num.clone();
            let mut num_max = num.clone();

            num_min.sort();
            num_max.sort_by(|a, b| b.cmp(a));

            let num_min = num_min
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i64>()
                .unwrap();
            let num_max = num_max
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i64>()
                .unwrap();

            let diff = num_max - num_min;
            num = diff.to_string().chars().collect::<Vec<_>>();

            if num.len() < 4 {
                for _ in 0..(4 - num.len()) {
                    num.insert(0, '0');
                }
            }

            ret += 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
