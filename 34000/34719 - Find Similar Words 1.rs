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

struct Myers {
    len_pattern: usize,
    num_words: usize,
    idx_msb: usize,
    mask_last: u64,
    peq: Vec<Vec<u64>>,
}

struct MyersState {
    pv: Vec<u64>,
    mv: Vec<u64>,
    ph: Vec<u64>,
    mh: Vec<u64>,
    score: i64,
    xh: Vec<u64>,
    ph_shift: Vec<u64>,
    mh_shift: Vec<u64>,
}

impl Myers {
    fn preprocess(pattern: &[u8]) -> Self {
        let len_pattern = pattern.len();
        let num_words = (len_pattern + 63) / 64;
        let idx_msb = (len_pattern - 1) & 63;
        let bits_last = ((len_pattern - 1) & 63) + 1;
        let mask_last = if bits_last == 64 {
            !0u64
        } else {
            (1u64 << bits_last) - 1
        };
        let mut peq = vec![vec![0; num_words]; 26];

        for (j, &c) in pattern.iter().enumerate() {
            let word = j / 64;
            let bit = j & 63;

            peq[(c - b'a') as usize][word] |= 1 << bit;
        }

        Self {
            len_pattern,
            num_words,
            idx_msb,
            mask_last,
            peq,
        }
    }

    fn init_state(&self) -> MyersState {
        let mut pv = vec![!0; self.num_words];
        pv[self.num_words - 1] = self.mask_last;

        MyersState {
            pv,
            mv: vec![0; self.num_words],
            xh: vec![0; self.num_words],
            ph: vec![0; self.num_words],
            mh: vec![0; self.num_words],
            ph_shift: vec![0; self.num_words],
            mh_shift: vec![0; self.num_words],
            score: self.len_pattern as i64,
        }
    }

    fn step(&self, state: &mut MyersState, c: u8) {
        let eq = &self.peq[(c - b'a') as usize];
        let mut carry_add = 0;

        for i in 0..self.num_words {
            let pv = state.pv[i];
            let mv = state.mv[i];
            let (s1, c1) = pv.overflowing_add((eq[i] | mv) & pv);
            let (s2, c2) = s1.overflowing_add(carry_add);

            carry_add = if c1 || c2 { 1 } else { 0 };

            let xh = (s2 ^ pv) | (eq[i] | mv);
            state.xh[i] = xh;
            state.ph[i] = mv | !(xh | pv);
            state.mh[i] = pv & xh;
        }

        if ((state.ph[self.num_words - 1] >> self.idx_msb) & 1) != 0 {
            state.score += 1;
        }

        if ((state.mh[self.num_words - 1] >> self.idx_msb) & 1) != 0 {
            state.score -= 1;
        }

        let mut carry_ph = 1;
        let mut carry_mh = 0;

        for i in 0..self.num_words {
            let ph = state.ph[i];
            let mh = state.mh[i];

            state.ph_shift[i] = (ph << 1) | carry_ph;
            carry_ph = ph >> 63;

            state.mh_shift[i] = (mh << 1) | carry_mh;
            carry_mh = mh >> 63;
        }

        for i in 0..self.num_words {
            let xh = state.xh[i];
            let ph_shift = state.ph_shift[i];
            let mh_shift = state.mh_shift[i];

            state.pv[i] = mh_shift | !(xh | ph_shift);
            state.mv[i] = ph_shift & xh;
        }

        state.pv[self.num_words - 1] &= self.mask_last;
        state.mv[self.num_words - 1] &= self.mask_last;
    }

    fn calculate(&self, s: &Vec<u8>, k: usize) -> Vec<u64> {
        let n = s.len();
        let mut ret = vec![0; k + 1];

        let min = self.len_pattern - k;
        let max = self.len_pattern + k;

        for i in 0..=n - min {
            let mut state = self.init_state();
            let len_max = max.min(n - i);

            if len_max < min {
                continue;
            }

            for j in 0..len_max {
                self.step(&mut state, s[i + j]);

                let len = j + 1;

                if len >= min {
                    if state.score >= 0 && state.score <= k as i64 {
                        ret[state.score as usize] += 1;
                    }

                    let remain = len_max - len;
                    let min_reachable = if state.score > remain as i64 {
                        state.score - remain as i64
                    } else {
                        0
                    };

                    if min_reachable > k as i64 {
                        break;
                    }
                }
            }
        }

        ret
    }
}

// Reference: https://www.gersteinlab.org/courses/452/09-spring/pdf/Myers.pdf
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (_, _, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let s = scan.token::<String>().as_bytes().to_vec();
    let t = scan.token::<String>().as_bytes().to_vec();

    let myers = Myers::preprocess(&t);
    let ret = myers.calculate(&s, k);

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
