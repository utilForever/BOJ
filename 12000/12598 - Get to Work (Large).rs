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

    let c = scan.token::<i64>();

    for i in 1..=c {
        let (n, t) = (scan.token::<usize>(), scan.token::<usize>() - 1);
        let mut towns = vec![(0, Vec::new()); n];

        let e = scan.token::<i64>();

        for _ in 0..e {
            let (h, p) = (scan.token::<usize>() - 1, scan.token::<i64>());

            if h == t {
                continue;
            }

            towns[h].0 += 1;

            if p != 0 {
                towns[h].1.push(p);
            }
        }

        write!(out, "Case #{i}: ").unwrap();

        let mut check = true;

        for (a, b) in towns.iter() {
            let sum = b.iter().sum::<i64>();

            if *a > sum {
                check = false;
                break;
            }
        }

        if !check {
            writeln!(out, "IMPOSSIBLE").unwrap();
            continue;
        }

        let mut ret = vec![0; n];

        for i in 0..n {
            towns[i].1.sort_unstable();

            let mut sum = towns[i].0;

            while sum > 0 {
                ret[i] += 1;
                sum -= towns[i].1.pop().unwrap();
            }
        }

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
