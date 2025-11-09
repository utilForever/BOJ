use io::Write;
use std::{cmp::Reverse, collections::BinaryHeap, io, str};

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

fn check(cnt: &Vec<i64>, mid: i64) -> bool {
    let mut active: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    let mut prev: Option<usize> = None;

    let finalize = |heap: &mut BinaryHeap<Reverse<i64>>| -> bool {
        while let Some(Reverse(len)) = heap.pop() {
            if len < mid {
                return false;
            }
        }

        true
    };

    for i in 1..cnt.len() {
        if cnt[i] == 0 {
            continue;
        }

        if let Some(idx) = prev {
            if i != idx + 1 && !finalize(&mut active) {
                return false;
            }
        }

        let mut next: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
        let m = active.len();

        if m <= cnt[i] as usize {
            for _ in 0..m {
                let Reverse(len) = active.pop().unwrap();
                next.push(Reverse(len + 1));
            }

            for _ in 0..cnt[i] as usize - m {
                next.push(Reverse(1));
            }
        } else {
            for _ in 0..cnt[i] as usize {
                let Reverse(len) = active.pop().unwrap();
                next.push(Reverse(len + 1));
            }

            if !finalize(&mut active) {
                return false;
            }
        }

        active = next;
        prev = Some(i);
    }

    finalize(&mut active)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for i in 1..=t {
        let n = scan.token::<usize>();
        let mut cnt = vec![0; 10001];

        for _ in 0..n {
            let card = scan.token::<usize>();
            cnt[card] += 1;
        }

        let mut left = 0;
        let mut right = n as i64;

        while left < right {
            let mid = (left + right + 1) / 2;

            if check(&cnt, mid) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }

        writeln!(out, "Case #{i}: {left}").unwrap();
    }
}
