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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut briefcases = [
        ("1", 0),
        ("5", 0),
        ("10", 0),
        ("20", 0),
        ("50", 0),
        ("100", 0),
    ];

    for i in 0..6 {
        briefcases[i].1 = scan.token::<i64>();
    }

    let mut briefcase_max = (String::from("1"), 0);

    for briefcase in briefcases.iter() {
        let amount = briefcase.1 * briefcase.0.parse::<i64>().unwrap();
        let amount_max = briefcase_max.1 * briefcase_max.0.parse::<i64>().unwrap();

        if amount > amount_max || amount == amount_max && briefcase.1 < briefcase_max.1 {
            briefcase_max = (briefcase.0.to_string(), briefcase.1);
        }
    }

    writeln!(out, "{}", briefcase_max.0).unwrap();
}
