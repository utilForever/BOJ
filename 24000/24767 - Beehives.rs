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

    loop {
        let (d, n) = (scan.token::<f64>(), scan.token::<usize>());

        if d == 0.0 && n == 0 {
            break;
        }

        let mut hives = vec![(0.0, 0.0); n];

        for i in 0..n {
            hives[i] = (scan.token::<f64>(), scan.token::<f64>());
        }

        let mut ret_sour = 0;
        let mut ret_sweet = 0;

        for i in 0..n {
            let mut is_fighting = false;

            for j in 0..n {
                if i == j {
                    continue;
                }

                if (hives[i].0 - hives[j].0).powi(2) + (hives[i].1 - hives[j].1).powi(2)
                    <= d.powi(2)
                {
                    is_fighting = true;
                    break;
                }
            }

            if is_fighting {
                ret_sour += 1;
            } else {
                ret_sweet += 1;
            }
        }

        writeln!(out, "{ret_sour} sour, {ret_sweet} sweet").unwrap();
    }
}
