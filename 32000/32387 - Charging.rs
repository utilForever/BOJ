use io::Write;
use std::{collections::BTreeSet, io, str};

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

    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut ports_status = vec![-1; n];
    let mut ports_available = BTreeSet::new();

    for i in 0..n {
        ports_available.insert(i);
    }

    for idx in 1..=q {
        let (cmd, i) = (scan.token::<i64>(), scan.token::<usize>());

        if cmd == 1 {
            if let Some(&pos) = ports_available.range((i - 1)..).next() {
                ports_status[pos] = idx;
                ports_available.remove(&pos);

                writeln!(out, "{}", pos + 1).unwrap();
            } else {
                writeln!(out, "-1").unwrap();
            }
        } else {
            if ports_status[i - 1] == -1 {
                writeln!(out, "-1").unwrap();
            } else {
                writeln!(out, "{}", ports_status[i - 1]).unwrap();

                ports_status[i - 1] = -1;
                ports_available.insert(i - 1);
            }
        }
    }
}
