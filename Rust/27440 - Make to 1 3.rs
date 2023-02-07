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

fn process_bfs(map: &mut HashMap<i64, i64>, n: i64) -> i64 {
    if map.contains_key(&n) {
        return *map.get(&n).unwrap();
    }

    let mut ret = Vec::new();

    if n % 3 == 0 {
        ret.push(process_bfs(map, n / 3));
    }

    if n % 2 == 0 {
        ret.push(process_bfs(map, n / 2));
    }

    if (n - 1) % 3 == 0 || (n - 1) % 2 == 0 {
        ret.push(process_bfs(map, n - 1));
    }

    if (n - 2) % 3 == 0 {
        ret.push(process_bfs(map, n - 2) + 1);
    }

    let min = ret.iter().min().unwrap();
    map.insert(n, min + 1);

    min + 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let mut ret = HashMap::new();

    ret.insert(1, 0);
    ret.insert(2, 1);
    ret.insert(3, 1);

    writeln!(out, "{}", process_bfs(&mut ret, n)).unwrap();
}
