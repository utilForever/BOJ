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

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        let val = max;

        max = min;
        min = val;
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn process_permutation_cycle_decomposition(vals: Vec<usize>, n: usize) -> (usize, usize) {
    let mut visited = vec![false; n + 1];
    let mut period = 1;
    let mut num_cycle = 0;

    for i in 1..=n {
        if !visited[i] {
            let mut k = 0_usize;
            let mut x = i;

            loop {
                if visited[x] {
                    break;
                }

                visited[x] = true;
                k += 1;
                x = vals[x];
            }

            period = period / gcd(period, k) * k;
            num_cycle += 1;
        }
    }

    (period, num_cycle)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut vals = vec![0; n + 1];

    for i in 1..=n {
        vals[i] = scan.token::<usize>();
    }

    let (period, _) = process_permutation_cycle_decomposition(vals, n);
    writeln!(out, "{}", period).unwrap();
}
