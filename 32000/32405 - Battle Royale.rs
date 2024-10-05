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
    let mut idxes = vec![0; n];
    let mut attacks = vec![0; n];
    let mut healths = vec![0; n];

    for i in 0..n {
        idxes[i] = i;
    }

    for i in 0..n {
        attacks[i] = scan.token::<i64>();
    }

    for i in 0..n {
        healths[i] = scan.token::<i64>();
    }

    let mut damage_sum = 0;

    while idxes.len() > 1 {
        let mut idxes_new = Vec::new();

        for i in 0..idxes.len() {
            if healths[idxes[i]] <= damage_sum {
                continue;
            }

            damage_sum += attacks[idxes[i]];
            healths[idxes[i]] += attacks[idxes[i]];
            idxes_new.push(idxes[i]);
        }

        std::mem::swap(&mut idxes, &mut idxes_new);
    }

    writeln!(out, "{}", idxes[0] + 1).unwrap();
}
