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
    let mut variables = [0; 4];

    for i in 0..4 {
        variables[i] = scan.token::<i64>();
    }

    let mut cards = vec![Vec::new(); 4];
    let mut idxes = vec![0; 4];

    for _ in 0..n {
        let (t, u) = (scan.token::<char>(), scan.token::<i64>());
        cards[(t as u8 - b'A') as usize].push(u);
    }

    for i in 0..4 {
        cards[i].sort_by(|a, b| b.cmp(a));
    }

    for _ in 0..k {
        let mut idx = 0;
        let mut val_max = 0.0;

        for i in 0..4 {
            if idxes[i] == cards[i].len() {
                continue;
            }

            let val = cards[i][idxes[i]] as f64 / variables[i] as f64;

            if val > val_max {
                idx = i;
                val_max = val;
            }
        }

        writeln!(
            out,
            "{} {}",
            (idx as u8 + b'A') as char,
            cards[idx][idxes[idx]]
        )
        .unwrap();

        variables[idx] += cards[idx][idxes[idx]];
        idxes[idx] += 1;
    }
}
