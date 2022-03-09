use io::Write;
use std::{cmp, io, str};

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
    let mut deposits = vec![(0, 0, 0); n];

    for i in 0..n {
        let (mut x0, mut x1, y) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
        }

        deposits[i] = (x0, x1, y);
    }

    let mut max_amount = 0;

    for i in 0..n {
        let mut candidates = Vec::new();
        let mut cur_amount = 0;

        for j in 0..n {
            if deposits[i].2 == deposits[j].2 {
                if deposits[i].0 >= deposits[j].0 && deposits[i].1 <= deposits[j].1 {
                    cur_amount += deposits[j].1 - deposits[j].0;
                }
            } else if deposits[i].2 > deposits[j].2 {
                candidates.push((
                    deposits[j].1 - deposits[j].0,
                    (deposits[i].0 - deposits[j].1, deposits[i].2 - deposits[j].2),
                ));
                candidates.push((
                    deposits[j].0 - deposits[j].1,
                    (deposits[i].0 - deposits[j].0, deposits[i].2 - deposits[j].2),
                ));
            } else {
                candidates.push((
                    deposits[j].1 - deposits[j].0,
                    (deposits[j].0 - deposits[i].0, deposits[j].2 - deposits[i].2),
                ));
                candidates.push((
                    deposits[j].0 - deposits[j].1,
                    (deposits[j].1 - deposits[i].0, deposits[j].2 - deposits[i].2),
                ));
            }
        }

        candidates.sort_by(|a, b| {
            let val_a = (a.1).0 * (b.1).1;
            let val_b = (b.1).0 * (a.1).1;

            if val_a != val_b {
                return val_a.cmp(&val_b);
            }

            a.0.cmp(&b.0).reverse()
        });

        max_amount = cmp::max(cur_amount, max_amount);

        for candidate in candidates.iter() {
            cur_amount += candidate.0;
            max_amount = cmp::max(cur_amount, max_amount);
        }
    }

    writeln!(out, "{}", max_amount).unwrap();
}
