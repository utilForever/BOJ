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
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<usize>();
    }

    if nums.iter().all(|&x| x == 0) {
        writeln!(out, "0").unwrap();

        for i in 0..n {
            write!(out, "{} ", nums[i]).unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    let mut visited = vec![false; n + 1];

    for i in 0..n {
        if nums[i] <= n {
            visited[nums[i]] = true;
        }
    }

    let mut m = 0;

    while visited[m] {
        m += 1;
    }

    let mut used = vec![false; n + 1];
    let mut left = Vec::new();
    let mut right = Vec::new();

    for i in 0..n {
        if nums[i] < m && !used[nums[i]] {
            used[nums[i]] = true;
            left.push(nums[i]);
        } else {
            right.push(nums[i]);
        }
    }

    left.sort_unstable();

    writeln!(out, "{}", m + 1).unwrap();

    for val in left {
        write!(out, "{val} ").unwrap();
    }

    for val in right {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
