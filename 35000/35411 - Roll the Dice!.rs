use io::Write;
use std::{collections::HashSet, io, str};

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

fn upper_bound(arr: &Vec<usize>, x: usize) -> usize {
    let (mut left, mut right) = (0, arr.len());

    while left < right {
        let mid = (left + right) / 2;

        if arr[mid] <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut desert_islands = vec![0; k];

    for i in 0..k {
        desert_islands[i] = scan.token::<usize>();
    }

    if n == 1 {
        let mut idx = 0;

        for i in 1..m {
            while idx < k && desert_islands[idx] < 2 * i {
                idx += 1;
            }

            if idx < k && desert_islands[idx] == 2 * i {
                writeln!(out, "{}", 2 * i).unwrap();
                return;
            }
        }

        writeln!(out, "0").unwrap();
        return;
    }

    let offset = 2 * n - 1;
    let blocked = desert_islands
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x / 2)
        .collect::<Vec<_>>();
    let mut set = HashSet::with_capacity(blocked.len() * 2 + 1);

    for &b in blocked.iter() {
        set.insert(b);
    }

    let mut pos = 0;
    let mut ret = offset;

    for _ in 1..m {
        let (left, right) = (pos + 1, pos + n);
        let ub = upper_bound(&blocked, right);

        if ub > 0 && blocked[ub - 1] >= left {
            ret = ret.max(2 * blocked[ub - 1]);
        }

        let mut pos_next = right;

        while set.contains(&pos_next) {
            pos_next -= 1;
        }

        if pos_next < left {
            break;
        }

        pos = pos_next;
        ret = ret.max(2 * pos + offset);
    }

    writeln!(out, "{ret}").unwrap();
}
