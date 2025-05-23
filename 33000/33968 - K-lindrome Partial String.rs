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

fn process_manachers(text: &str) -> Vec<usize> {
    let mut s = String::from('*');

    for c in text.chars() {
        s.push(c);
        s.push('*');
    }

    let s = s.chars().collect::<Vec<_>>();
    let mut ret = vec![0; s.len()];
    let mut r = 0;
    let mut c = 0;

    for i in 0..s.len() {
        ret[i] = if r < i { 0 } else { ret[2 * c - i].min(r - i) };

        while i as i64 - ret[i] as i64 - 1 >= 0
            && i + ret[i] + 1 < s.len()
            && s[i - ret[i] - 1] == s[i + ret[i] + 1]
        {
            ret[i] += 1;
        }

        if r < i + ret[i] {
            r = i + ret[i];
            c = i;
        }
    }

    ret
}

const BASE: u64 = 911_382_323;

fn build_rolling_hash(s: &Vec<u8>) -> (Vec<u64>, Vec<u64>) {
    let n = s.len();
    let mut prefix = vec![0_u64; n + 1];
    let mut pow = vec![1_u64; n + 1];

    for i in 0..n {
        let v = (s[i] - b'a' + 1) as u64;

        prefix[i + 1] = prefix[i].wrapping_mul(BASE).wrapping_add(v);
        pow[i + 1] = pow[i].wrapping_mul(BASE);
    }

    (prefix, pow)
}

fn hash(prefix: &Vec<u64>, pow: &Vec<u64>, left: usize, right: usize) -> u64 {
    let len = right - left + 1;
    prefix[right + 1].wrapping_sub(prefix[left].wrapping_mul(pow[len]))
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let _ = scan.token::<i64>();
    let s = scan.token::<String>().bytes().collect::<Vec<_>>();
    let (prefix, pow) = build_rolling_hash(&s);
    let n = s.len();

    for k in 1..=n {
        let mut ret = 0;

        for offset in 0..k {
            let block_cnt = (n - offset) / k;

            if block_cnt == 0 {
                continue;
            }

            let mut map = HashMap::new();
            let mut next = 0x1000;
            let mut block_str = String::with_capacity(block_cnt);

            for b in 0..block_cnt {
                let left = offset + b * k;
                let right = left + k - 1;
                let hash = hash(&prefix, &pow, left, right);

                let ch = *map.entry(hash).or_insert_with(|| {
                    let ch = std::char::from_u32(next).unwrap();
                    next += 1;
                    ch
                });

                block_str.push(ch);
            }

            let val = process_manachers(&block_str);
            ret += val.iter().map(|&x| ((x + 1) / 2) as i64).sum::<i64>();
        }

        writeln!(out, "{ret}").unwrap();
    }
}
