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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let s = scan.token::<String>();
    let manacher = process_manachers(&s);
    let mut ret = vec![0i64; s.len() + 1];

    let check_range = |idx: i64| -> bool { idx >= 1 && idx <= s.len() as i64 };

    for i in 0..manacher.len() {
        if manacher[i] % 2 == 0 {
            let idx1 = (i as i64 - manacher[i] as i64) / 2 + 1;
            let idx2 = (i as i64) / 2 + 1;
            let idx3 = idx2 + 1;
            let idx4 = idx1 + manacher[i] as i64 + 1;

            if check_range(idx1) {
                ret[idx1 as usize] += 1;
            }

            if check_range(idx2) {
                ret[idx2 as usize] -= 1;
            }

            if check_range(idx3) {
                ret[idx3 as usize] -= 1;
            }

            if check_range(idx4) {
                ret[idx4 as usize] += 1;
            }
        } else {
            let idx1 = i as i64 / 2 - (manacher[i] as i64) / 2 + 1;
            let idx2 = i as i64 / 2 + 2;
            let idx3 = idx1 + manacher[i] as i64 + 1;

            if check_range(idx1) {
                ret[idx1 as usize] += 1;
            }

            if check_range(idx2) {
                ret[idx2 as usize] -= 2;
            }

            if check_range(idx3) {
                ret[idx3 as usize] += 1;
            }
        }
    }

    for i in 1..ret.len() {
        ret[i] += ret[i - 1];
    }

    for i in 1..ret.len() {
        ret[i] += ret[i - 1];
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let x = scan.token::<usize>();
        writeln!(out, "{}", ret[x]).unwrap();
    }
}
