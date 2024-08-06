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

    let n = scan.token::<usize>();
    let mut piles = vec![String::new(); n];
    let mut hackenbush = vec![(0.0, 0); n];

    for i in 0..n {
        piles[i] = scan.token::<String>();
    }

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    for (idx, pile) in piles.iter().enumerate() {
        let pile = pile.chars().collect::<Vec<_>>();
        let mut is_same = true;
        let mut val = 1.0;
        let mut ret = 0.0;

        for i in 0..pile.len() {
            if i > 0 && pile[i] != pile[i - 1] {
                is_same = false;
            }

            if !is_same {
                val *= 0.5;
            }

            ret += if pile[i] == 'W' { 1.0 } else { -1.0 } * val;
        }

        hackenbush[idx] = (ret, pile.len());
    }

    let calculate = |hackenbush: Vec<(f64, usize)>, n: usize| -> Vec<(f64, usize)> {
        let mut ret = Vec::new();
        ret.push((0.0, 0));

        for i in 0..n {
            let mut acc = ret
                .iter()
                .map(|&(val, len)| (val + hackenbush[i].0, len + hackenbush[i].1))
                .collect::<Vec<_>>();

            ret.append(&mut acc);
        }

        ret.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        ret.dedup();

        ret
    };

    let left = calculate(hackenbush[0..n / 2].to_vec(), n / 2);
    let right = calculate(hackenbush[n / 2..n].to_vec(), n - n / 2);
    let mut idx_left = 0;
    let mut idx_right = right.len() - 1;
    let mut ret = 0;

    while idx_left < left.len() {
        while idx_right > 0 && left[idx_left].0 + right[idx_right].0 < 0.0 {
            idx_right -= 1;
        }

        if left[idx_left].0 + right[idx_right].0 == 0.0 {
            ret = ret.max(left[idx_left].1 + right[idx_right].1);
        }

        idx_left += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
