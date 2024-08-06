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

fn process_permutation_cycle_decomposition(vals: Vec<usize>, n: usize) -> usize {
    let mut visited = vec![false; n];
    let mut seq = vec![0; n];
    let mut ret = 0;

    for i in 0..n {
        seq[i] = i;
    }

    seq.sort_by_key(|&x| vals[x]);

    for i in 0..n {
        if !visited[i] {
            let mut picked_grumpinesses = Vec::new();
            let mut x = i;

            loop {
                if visited[x] {
                    break;
                }

                visited[x] = true;
                picked_grumpinesses.push(vals[x]);
                x = seq[x];
            }

            ret += cmp::min(
                picked_grumpinesses.iter().sum::<usize>()
                    + picked_grumpinesses.iter().min().unwrap() * (picked_grumpinesses.len() - 2),
                picked_grumpinesses.iter().sum::<usize>()
                    + picked_grumpinesses.iter().min().unwrap()
                    + vals.iter().min().unwrap() * (picked_grumpinesses.len() + 1),
            );
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut grumpinesses = vec![0; n];

    for i in 0..n {
        grumpinesses[i] = scan.token::<usize>();
    }

    let ret = process_permutation_cycle_decomposition(grumpinesses, n);
    writeln!(out, "{}", ret).unwrap();
}
