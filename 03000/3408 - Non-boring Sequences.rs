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
}

fn is_non_boring(pos_prev: &Vec<i64>, pos_next: &Vec<i64>, start: i64, end: i64) -> bool {
    if start >= end {
        return true;
    }

    let mut left = start;
    let mut right = end;
    let check = |start: i64, end: i64, idx: i64| -> bool {
        pos_prev[idx as usize] < start && pos_next[idx as usize] > end
    };

    while left <= right {
        if check(start, end, left) {
            return is_non_boring(pos_prev, pos_next, start, left - 1)
                && is_non_boring(pos_prev, pos_next, left + 1, end);
        } else if check(start, end, right) {
            return is_non_boring(pos_prev, pos_next, start, right - 1)
                && is_non_boring(pos_prev, pos_next, right + 1, end);
        }

        left += 1;
        right -= 1;
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut nums = HashMap::new();
        let mut pos_prev = vec![-1; n];
        let mut pos_next = vec![n as i64; n];

        for i in 0..n {
            let val = scan.token::<i64>();

            if let Some(&pos) = nums.get(&val) {
                pos_prev[i] = pos as i64;
                pos_next[pos] = i as i64;
            }

            nums.insert(val, i);
        }

        writeln!(
            out,
            "{}",
            if is_non_boring(&pos_prev, &pos_next, 0, n as i64 - 1) {
                "non-boring"
            } else {
                "boring"
            }
        )
        .unwrap();
    }
}
