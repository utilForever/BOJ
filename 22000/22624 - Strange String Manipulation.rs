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
    let mut input = vec![0; n];

    for i in 0..n {
        input[i] = scan.token::<i64>();
    }

    let mut entropy_min = f64::MAX;
    let mut ret = (0, 0, 0);

    for s in 0..=15 {
        for a in 0..=15 {
            for c in 0..=15 {
                let mut random = vec![0; n + 1];
                random[0] = s;

                for i in 1..=n {
                    random[i] = (a * random[i - 1] + c) % 256;
                }

                let mut output = vec![0; n];

                for i in 0..n {
                    output[i] = (input[i] + random[i + 1]) % 256;
                }

                let mut alphabet = vec![0; 256];

                for idx in output {
                    alphabet[idx as usize] += 1;
                }

                let mut entropy = 0.0;

                for cnt in alphabet {
                    if cnt == 0 {
                        continue;
                    }

                    let p = cnt as f64 / n as f64;
                    entropy -= p * p.log2();
                }

                if entropy < entropy_min {
                    entropy_min = entropy;
                    ret = (s, a, c);
                }
            }
        }
    }

    writeln!(out, "{} {} {}", ret.0, ret.1, ret.2).unwrap();
}
