use io::Write;
use std::{
    collections::{BTreeMap, HashSet},
    io, str,
};

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

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut points = vec![(0, 0); n];

    for i in 0..n {
        points[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    points.sort_by_key(|x| x.1);

    let mut map: BTreeMap<i64, HashSet<i64>> = BTreeMap::new();
    let mut ret: Option<i64> = None;

    for (a, b1) in points {
        let diff = k - b1;
        let mut range = map.range(..=diff).next_back();

        while let Some((b2, set)) = range {
            if set.len() > 1 || !set.contains(&a) {
                let sum = b1 + b2;

                if let Some(cur) = ret {
                    if sum > cur {
                        ret = Some(sum);
                    }
                } else {
                    ret = Some(sum);
                }

                break;
            }

            range = map.range(..b2).next_back();
        }

        map.entry(b1).or_insert_with(HashSet::new).insert(a);
    }

    match ret {
        Some(val) => writeln!(out, "{val}"),
        None => writeln!(out, "NO"),
    }
    .unwrap();
}
