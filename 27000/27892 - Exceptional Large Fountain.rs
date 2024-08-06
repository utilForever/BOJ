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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (x0, n) = (scan.token::<i64>(), scan.token::<i64>());
    let mut hash_set = HashSet::new();
    let mut vec = Vec::new();
    let mut x = x0;
    let mut is_unique = true;
    let mut idx = 0;

    loop {
        x = if x % 2 == 0 { (x / 2) ^ 6 } else { (2 * x) ^ 6 };

        if hash_set.contains(&x) {
            is_unique = false;
            break;
        } else {
            vec.push(x);
            hash_set.insert(x);
        }

        idx += 1;

        if idx == n {
            break;
        }
    }

    if is_unique {
        writeln!(out, "{x}").unwrap();
    } else {
        let pos = vec.iter().position(|&val| val == x).unwrap();
        let vec = vec[pos..].to_vec();
        
        writeln!(out, "{}", vec[(n - idx - 1) as usize % vec.len()]).unwrap();
    }
}
