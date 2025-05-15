use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (n, k) = (scan.token::<usize>(), scan.token::<usize>());
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut prefix_sum = vec![0; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + nums[i];
    }

    let mut best_diff = i64::MIN;
    let mut best_p1 = 1;
    let mut best_p2 = 1;

    // Case 1: p1 <= p2
    let mut deque = VecDeque::new();

    for i in 1..=n {
        let pos = i - 1;
        let val = prefix_sum[pos];

        while let Some(&(_, back_val)) = deque.back() {
            if back_val > val {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back((pos, val));

        let bound = if i >= k + 1 { i - (k + 1) } else { 0 };

        while let Some(&(front_pos, _)) = deque.front() {
            if front_pos < bound {
                deque.pop_front();
            } else {
                break;
            }
        }

        let &(pos0, val0) = deque.front().unwrap();
        let diff = prefix_sum[i] - val0;
        let p1 = pos0 + 1;
        let p2 = i;

        if diff > best_diff
            || (diff == best_diff && (p1 < best_p1 || (p1 == best_p1 && p2 < best_p2)))
        {
            best_diff = diff;
            best_p1 = p1;
            best_p2 = p2;
        }
    }

    // Case 2: p1 > p2
    let mut deque = VecDeque::new();

    for i in 1..n {
        let pos = i;
        let val = prefix_sum[pos];

        while let Some(&(_, back_val)) = deque.back() {
            if back_val > val {
                deque.pop_back();
            } else {
                break;
            }
        }

        deque.push_back((pos, val));

        let bound = if i + 1 > k { i + 1 - k } else { 1 };

        while let Some(&(front_pos, _)) = deque.front() {
            if front_pos < bound {
                deque.pop_front();
            } else {
                break;
            }
        }

        let &(pos0, val0) = deque.front().unwrap();
        let diff = prefix_sum[i] - val0;
        let p1 = i + 1;
        let p2 = pos0;

        if diff > best_diff
            || (diff == best_diff && (p1 < best_p1 || (p1 == best_p1 && p2 < best_p2)))
        {
            best_diff = diff;
            best_p1 = p1;
            best_p2 = p2;
        }
    }

    writeln!(out, "{}", -prefix_sum[n] + 2 * best_diff).unwrap();
    writeln!(out, "{best_p1} {best_p2}").unwrap();
}
