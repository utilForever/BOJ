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

const MOD: i64 = 1_000_000_007;

#[inline(always)]
fn pow(x: i64, mut y: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % MOD;

    while y != 0 {
        if y & 1 != 0 {
            ret = (ret * piv) % MOD;
        }

        piv = (piv * piv) % MOD;
        y >>= 1;
    }

    ret
}

// Thanks for @lem0nad3 to provide the important idea of the solution.
// Reference: https://en.wikipedia.org/wiki/Golomb_sequence
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut golomb = vec![0; 35_000_001];

    golomb[1] = 1;

    let mut idx = 1;
    let mut acc = 1;

    loop {
        idx += 1;
        golomb[idx] = 1 + golomb[idx - golomb[golomb[idx - 1]]];
        acc += golomb[idx];

        if acc > n {
            golomb[idx] = n - acc + golomb[idx];
            break;
        }
    }

    let mut inverse = vec![0; 35_000_001];

    for i in 1..=golomb[idx - 1].max(golomb[idx]) {
        inverse[i] = 1;
    }

    for i in 1..=idx {
        inverse[golomb[i]] = (inverse[golomb[i]] * i as i64) % MOD;
    }

    let mut ret = 1;

    for i in 1..=golomb[idx].max(golomb[idx - 1]) {
        ret = (ret * pow(inverse[i], i as i64)) % MOD;
    }

    writeln!(out, "{ret}").unwrap();
}
