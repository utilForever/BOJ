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

    let (n, a, b) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut stones = vec![0; n];

    for i in 0..n {
        stones[i] = scan.token::<usize>();
    }

    let min = cmp::min(a, b);
    let mut grundy = 0;
    let mut num_exceed_piles = 0;

    for i in 0..n {
        grundy ^= stones[i] % (min + 1);

        if stones[i] > min {
            num_exceed_piles += 1;
        }
    }

    if num_exceed_piles == 0 || a == b {
        // Case 1: If each of piles is less than min(a, b) or a == b, then the answer is the grundy number.
        writeln!(out, "{}", if grundy > 0 { "Petyr" } else { "Varys" }).unwrap();
    } else if a > b {
        // Case 2: If a > b and there are piles that are greater than min(a, b), then the answer is Petyr.
        writeln!(out, "Petyr").unwrap();
    } else {
        // Case 3: If a < b and there are piles that are greater than min(a, b), then calculate the next situation.
        let max_stone = stones.iter().max().unwrap();
        let next_grundy = grundy ^ (max_stone % (a + 1));
        let remain = (((max_stone - next_grundy) % (a + 1)) + a + 1) % (a + 1);

        if (remain == 0 || remain > a || max_stone - remain > a)
            || (next_grundy != (max_stone - remain) % (a + 1))
        {
            writeln!(out, "Varys").unwrap();
        } else {
            writeln!(out, "Petyr").unwrap();
        }
    }
}
