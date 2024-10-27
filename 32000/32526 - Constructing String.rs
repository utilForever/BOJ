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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());

    if (n % (k + 1) != 0 || n / (k + 1) == 2) && (k == 0 || n % k != 0) {
        writeln!(out, "No").unwrap();
        return;
    }

    writeln!(out, "Yes").unwrap();

    if n % (k + 1) == 0 {
        let period = n / (k + 1);
        let mut ret = vec!['a'; period];

        ret[period / 2] = 'b';

        if period % 2 == 0 {
            ret[period / 2 - 1] = 'b';
        }

        for _ in 0..=k {
            write!(out, "{}", ret.iter().collect::<String>()).unwrap();
        }

        writeln!(out).unwrap();
    } else {
        let period = n / k;
        let mut ret = vec!['a'; period];

        ret[period - 1] = 'b';

        for _ in 0..k {
            write!(out, "{}", ret.iter().collect::<String>()).unwrap();
        }

        writeln!(out).unwrap();
    }
}
