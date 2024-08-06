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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
        let mut power_sejun = vec![0; n];
        let mut power_sebi = vec![0; m];

        for i in 0..n {
            power_sejun[i] = scan.token::<i64>();
        }

        for i in 0..m {
            power_sebi[i] = scan.token::<i64>();
        }

        power_sejun.sort_unstable_by(|a, b| b.cmp(a));
        power_sebi.sort_unstable_by(|a, b| b.cmp(a));

        while !power_sejun.is_empty() && !power_sebi.is_empty() {
            let val_sejun = *power_sejun.last().unwrap();
            let val_sebi = *power_sebi.last().unwrap();

            if val_sejun < val_sebi {
                power_sejun.pop();
            } else {
                power_sebi.pop();
            }
        }

        writeln!(out, "{}", if power_sejun.is_empty() { "B" } else { "S" }).unwrap();
    }
}
