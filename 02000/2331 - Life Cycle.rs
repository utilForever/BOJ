use io::Write;
use std::{collections::HashSet, io, str};

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

    let (a, p) = (scan.token::<i64>(), scan.token::<u32>());
    let mut set = HashSet::new();
    let mut val = a;
    let mut idx = 1;

    set.insert((val, idx));

    loop {
        let mut val_new = 0;

        while val > 0 {
            val_new += (val % 10).pow(p);
            val /= 10;
        }

        val = val_new;
        idx += 1;

        if set.iter().map(|x| x.0).any(|x| x == val) {
            break;
        }

        set.insert((val, idx));
    }

    let ret = set
        .iter()
        .filter(|x| x.0 == val)
        .map(|x| x.1)
        .collect::<Vec<_>>();

    writeln!(out, "{}", ret[0] - 1).unwrap();
}
