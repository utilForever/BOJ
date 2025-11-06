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

    let n = scan.token::<usize>();
    let mut ratios = vec![0; n];

    for i in 0..n {
        ratios[i] = scan.token::<usize>();
    }

    if n == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let sum = ratios.iter().sum::<usize>();
    let mut can_make = vec![false; sum + 1];
    let mut parent = vec![usize::MAX; sum + 1];
    let mut prev = vec![usize::MAX; sum + 1];

    can_make[0] = true;

    for (idx, &ratio) in ratios.iter().enumerate() {
        for s in (ratio..=sum).rev() {
            if can_make[s - ratio] && !can_make[s] {
                can_make[s] = true;
                parent[s] = idx;
                prev[s] = s - ratio;
            }
        }
    }

    let mut best_val = 0;
    let mut best_diff = sum + 1;

    for val in 1..sum {
        if !can_make[val] {
            continue;
        }

        let diff = if 2 * val >= sum {
            2 * val - sum
        } else {
            sum - 2 * val
        };

        if diff < best_diff || (diff == best_diff && val >= sum - val) {
            best_diff = diff;
            best_val = val;
        }
    }

    let mut used = vec![false; n];
    let mut curr = best_val;

    while curr != 0 {
        let idx = parent[curr];

        used[idx] = true;
        curr = prev[curr];
    }

    let mut ret1 = Vec::new();
    let mut ret2 = Vec::new();

    for i in 0..n {
        if used[i] {
            ret1.push(i + 1);
        } else {
            ret2.push(i + 1);
        }
    }

    writeln!(out, "{}", ret1.len()).unwrap();

    for val in ret1 {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();

    writeln!(out, "{}", ret2.len()).unwrap();

    for val in ret2 {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
