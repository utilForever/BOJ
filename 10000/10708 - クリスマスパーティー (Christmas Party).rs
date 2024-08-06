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

    let n = scan.token::<usize>();
    let m = scan.token::<usize>();
    let mut targets = vec![0; m + 1];

    for i in 1..=m {
        targets[i] = scan.token::<usize>();
    }

    let mut ret = vec![0; n + 1];

    for i in 1..=m {
        ret[targets[i]] += 1;
        
        let mut cnt = 0;

        for j in 1..=n {
            let vote = scan.token::<usize>();

            if targets[i] == j {
                continue;
            }

            if vote == targets[i] {
                ret[j] += 1;
            } else {
                cnt += 1;
            }
        }

        ret[targets[i]] += cnt;
    }

    for i in 1..=n {
        writeln!(out, "{}", ret[i]).unwrap();
    }
}
