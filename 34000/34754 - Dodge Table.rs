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

    pub fn all(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_to_string(&mut input).expect("Failed read");
        input
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

    let k = scan.token::<usize>();
    let mut neighbors = vec![Vec::new(); 16];

    for r in 0..4 {
        for c in 0..4 {
            let idx = r * 4 + c;
            neighbors[idx].push(idx);

            if r > 0 {
                neighbors[idx].push((r - 1) * 4 + c);
            }

            if r + 1 < 4 {
                neighbors[idx].push((r + 1) * 4 + c);
            }

            if c > 0 {
                neighbors[idx].push(r * 4 + (c - 1));
            }

            if c + 1 < 4 {
                neighbors[idx].push(r * 4 + (c + 1));
            }
        }
    }

    let mut attacks: Vec<Vec<u16>> = vec![Vec::new(); k + 2];
    let mut cnt = [0; 16];
    let mut dp_curr = [0; 16];
    let mut dp_next = [0; 16];

    for t in 1..=k {
        for &mask in attacks[t].iter() {
            let mut mask = mask;

            while mask != 0 {
                let idx = mask.trailing_zeros() as usize;

                cnt[idx] -= 1;
                mask &= mask - 1;
            }
        }

        let (d, s, len, p) = (
            scan.token::<char>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
            scan.token::<usize>(),
        );
        let mut mask = 0;

        match d {
            'U' => {
                let c = s - 1;

                for rr in 0..len {
                    mask |= 1u16 << (rr * 4 + c);
                }
            }
            'D' => {
                let c = s - 1;

                for rr in (4 - len)..4 {
                    mask |= 1u16 << (rr * 4 + c);
                }
            }
            'L' => {
                let r = s - 1;

                for cc in 0..len {
                    mask |= 1u16 << (r * 4 + cc);
                }
            }
            'R' => {
                let r = s - 1;

                for cc in (4 - len)..4 {
                    mask |= 1u16 << (r * 4 + cc);
                }
            }
            _ => unreachable!(),
        }

        {
            let mut mask = mask;

            while mask != 0 {
                let idx = mask.trailing_zeros() as usize;

                cnt[idx] += 1;
                mask &= mask - 1;
            }
        }

        if t + p <= k {
            attacks[t + p].push(mask);
        }

        for i in 0..16 {
            let hazard = if cnt[i] > 0 { 1 } else { 0 };
            let mut best = i64::MAX;

            for &j in neighbors[i].iter() {
                best = best.min(dp_curr[j]);
            }

            dp_next[i] = best + hazard;
        }

        dp_curr = dp_next;
    }

    writeln!(out, "{}", dp_curr.iter().min().unwrap()).unwrap();
}
