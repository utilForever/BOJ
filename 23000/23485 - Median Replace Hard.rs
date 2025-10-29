use io::Write;
use std::{collections::HashMap, io, str};

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

const MOD: i64 = 1_000_000_007;

struct DFA<const K: usize> {
    n: usize,
    start: usize,
    accept: Vec<bool>,
    transition: Vec<[usize; K]>,
}

impl<const K: usize> DFA<K> {
    fn new(n: usize, start: usize) -> Self {
        Self {
            n,
            start,
            accept: vec![false; n],
            transition: vec![[0; K]; n],
        }
    }

    fn count(&self, s: &str) -> i64 {
        let mut dp = vec![0; self.n];
        dp[self.start] = 1;

        for b in s.bytes() {
            let mut dp_new = vec![0; self.n];

            match b {
                b'0' => {
                    for (state, &ways) in dp.iter().enumerate() {
                        if ways == 0 {
                            continue;
                        }

                        let to = self.transition[state][0];
                        dp_new[to] = (dp_new[to] + ways) % MOD;
                    }
                }
                b'1' => {
                    for (state, &ways) in dp.iter().enumerate() {
                        if ways == 0 {
                            continue;
                        }

                        let to = self.transition[state][1];
                        dp_new[to] = (dp_new[to] + ways) % MOD;
                    }
                }
                b'?' => {
                    for (state, &ways) in dp.iter().enumerate() {
                        if ways == 0 {
                            continue;
                        }

                        let to0 = self.transition[state][0];
                        let to1 = self.transition[state][1];
                        dp_new[to0] = (dp_new[to0] + ways) % MOD;
                        dp_new[to1] = (dp_new[to1] + ways) % MOD;
                    }
                }
                _ => unreachable!("input must be 0/1/?"),
            }

            dp = dp_new;
        }

        let mut ret = 0;

        for (state, &ways) in dp.iter().enumerate() {
            if self.accept[state] {
                ret = (ret + ways) % MOD;
            }
        }

        ret
    }
}

const BIT_WINDOW: usize = 3;
const LIMIT_PREFIX: usize = 9;
const LIMIT_LOOKAHEAD: usize = 6;

fn idx(len: usize, s: usize) -> usize {
    (1 << len) | s
}

fn bit_len(mut x: usize) -> usize {
    let mut len = 0;

    while x > 0 {
        len += 1;
        x >>= 1;
    }

    len
}

// Build a DFA that accepts binary strings which evaluate to a given string
//   0b000 → "00"
//   0b001 → "01"
//   0b010 → "10"
//   0b011 → "11"
//   0b100 → "0"
//   0b101 → "1"
fn build_dfa(mut pattern: [u8; 8]) -> DFA<2> {
    pattern.swap(1, 4);
    pattern.swap(3, 6);

    // Compute which bit strings are accepted using dynamic programming
    let mut accept = vec![false; 1 << (LIMIT_PREFIX + LIMIT_LOOKAHEAD + 1)];
    accept[idx(1, 1)] = true;

    for len in (3..=LIMIT_PREFIX + LIMIT_LOOKAHEAD).step_by(2) {
        for s in 0..(1 << len) {
            let mut check = false;

            for i in 0..=len - BIT_WINDOW {
                let k = (s >> i) & 7;
                let bit = (pattern[k] as usize) & 1;
                let left = s & ((1 << i) - 1);
                let right = s >> (i + BIT_WINDOW);
                let prefix = ((right << 1) | bit) << i | left;

                if accept[idx(len - 2, prefix)] {
                    check = true;
                    break;
                }
            }

            if check {
                accept[idx(len, s)] = true;
            }
        }
    }

    let mut map = HashMap::new();
    let mut representatives = Vec::new();
    let mut states_accept = Vec::new();

    for len in 0..=LIMIT_PREFIX {
        for state in 1 << len..2 << len {
            let mut key = vec![0; 1 << (LIMIT_LOOKAHEAD + 1)];

            for next in 0..=LIMIT_LOOKAHEAD {
                let base = 1 << next;

                for z in 0..1 << next {
                    key[base | z] = accept[idx(len + next, (state << next) | z)] as u8;
                }
            }

            if !map.contains_key(&key) {
                let id = map.len();

                map.insert(key.clone(), id);
                representatives.push(state);
                states_accept.push(key[1] != 0);
            }
        }
    }

    // Construct a DFA using the Myhill-Nerode theorem
    let n = representatives.len();
    let mut dfa = DFA::<2>::new(n, 0);
    let mut key_start = vec![0; 1 << (LIMIT_LOOKAHEAD + 1)];

    for next in 0..=LIMIT_LOOKAHEAD {
        let len = 0;
        let state = 1;
        let base = 1 << next;

        for z in 0..(1 << next) {
            key_start[base | z] = accept[idx(len + next, (state << next) | z)] as u8;
        }
    }

    // Build transitions
    for (id, &state) in representatives.iter().enumerate() {
        for c in 0..=1 {
            let state2 = (state << 1) | c;
            let len2 = bit_len(state2) - 1;
            let mut key2 = vec![0u8; 1 << (LIMIT_LOOKAHEAD + 1)];

            for next in 0..=LIMIT_LOOKAHEAD {
                let base = 1 << next;

                for z in 0..(1 << next) {
                    key2[base | z] = accept[idx(len2 + next, (state2 << next) | z)] as u8;
                }
            }

            dfa.transition[id][c] = *map.get(&key2).unwrap();
        }
    }

    dfa.start = *map.get(&key_start).unwrap();
    dfa.accept = states_accept;

    dfa
}

// Reference: https://oi-wiki.org/misc/fsm/#myhillnerode-%E5%AE%9A%E7%90%86
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (p, s) = (scan.token::<String>(), scan.token::<String>());
        let mut pattern = [0; 8];

        for (i, c) in p.chars().enumerate() {
            pattern[i] = if c == '0' { 0 } else { 1 };
        }

        let dfa = build_dfa(pattern);
        let ret = dfa.count(&s);

        writeln!(out, "{ret}").unwrap();
    }
}
