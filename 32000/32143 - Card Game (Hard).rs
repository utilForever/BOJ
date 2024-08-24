use io::Write;
use std::{collections::BinaryHeap, io, str};

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

    let h = scan.token::<i64>();
    let (n, q) = (scan.token::<usize>(), scan.token::<i64>());
    let mut cards = vec![0; n];

    for i in 0..n {
        cards[i] = scan.token::<i64>();
    }

    cards.sort_by(|a, b| b.cmp(a));

    let mut bags = BinaryHeap::new();
    let mut min = i64::MAX;
    let mut sum = 0;

    for i in 0..n {
        bags.push(cards[i]);
        sum += cards[i];
        min = min.min(cards[i]);

        if sum >= h {
            break;
        }
    }

    if sum >= h {
        writeln!(out, "{}", bags.len()).unwrap();
    } else {
        writeln!(out, "-1").unwrap();
    }

    for _ in 0..q {
        let x = scan.token::<i64>();

        if sum >= h && x <= min {
            writeln!(out, "{}", bags.len()).unwrap();
            continue;
        }

        bags.push(x);
        sum += x;
        min = min.min(x);

        if sum < h {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut bags_new = BinaryHeap::new();
        min = i64::MAX;
        sum = 0;

        for _ in 0..bags.len() {
            let y = bags.pop().unwrap();
            sum += y;
            min = min.min(y);

            bags_new.push(y);

            if sum >= h {
                break;
            }
        }

        bags = bags_new;

        if sum >= h {
            writeln!(out, "{}", bags.len()).unwrap();
        } else {
            writeln!(out, "-1").unwrap();
        }
    }
}
