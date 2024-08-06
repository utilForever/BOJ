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

fn travasal_tree(
    buildings: &Vec<i64>,
    ret: &mut Vec<Vec<i64>>,
    level: usize,
    start: usize,
    end: usize,
) {
    if start == end {
        return;
    }

    let mid = (start + end) / 2;

    travasal_tree(buildings, ret, level + 1, start, mid);
    ret[level].push(buildings[mid]);
    travasal_tree(buildings, ret, level + 1, mid + 1, end);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let k = scan.token::<u32>();
    let mut buildings = vec![0; 2_usize.pow(k) + 1];

    for i in 1..=2_usize.pow(k) - 1 {
        buildings[i] = scan.token::<i64>();
    }

    let mut ret = vec![Vec::new(); k as usize + 1];

    travasal_tree(&buildings, &mut ret, 1, 1, 2_usize.pow(k));

    for i in 1..=k as usize {
        for j in 0..ret[i].len() {
            write!(out, "{} ", ret[i][j]).unwrap();
        }
        writeln!(out, "").unwrap();
    }
}
