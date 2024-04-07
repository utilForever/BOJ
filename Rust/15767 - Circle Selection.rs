use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet},
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
}

// Reference: https://codeforces.com/blog/entry/59650
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut circles = vec![(0, 0, 0, 0); n];
    let mut idxes = BTreeSet::new();

    for i in 0..n {
        circles[i] = (
            i + 1,
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        idxes.insert(i + 1);
    }

    circles.sort_by(|a, b| {
        if a.3 == b.3 {
            a.0.cmp(&b.0)
        } else {
            b.3.cmp(&a.3)
        }
    });

    circles.insert(0, (0, 0, 0, 0));

    let mut square_size = 0;
    let mut map = BTreeMap::new();
    let mut ret = vec![0; n + 1];

    let rescaling = |map: &mut BTreeMap<(i64, i64), BTreeSet<usize>>,
                     idxes: &BTreeSet<usize>,
                     square_size: i64| {
        map.clear();

        for &idx in idxes.iter() {
            let x = circles[idx].1 / square_size;
            let y = circles[idx].2 / square_size;

            map.entry((x, y)).or_insert(BTreeSet::new()).insert(idx);
        }
    };

    for i in 1..=n {
        if ret[circles[i].0 as usize] != 0 {
            continue;
        }

        if square_size == 0 || circles[i].3 * 2 < square_size {
            square_size = circles[i].3;
            rescaling(&mut map, &idxes, square_size);
        }

        for dx in circles[i].1 / square_size - 2..=circles[i].1 / square_size + 2 {
            for dy in circles[i].2 / square_size - 2..=circles[i].2 / square_size + 2 {
                if let Some(candidates) = map.get(&(dx, dy)) {
                    for idx in candidates.iter() {
                        if ret[circles[*idx].0] != 0 {
                            continue;
                        }

                        let x = circles[i].1 - circles[*idx].1;
                        let y = circles[i].2 - circles[*idx].2;

                        // Check if the circle is inside the other circle
                        if x * x + y * y <= (circles[i].3 + circles[*idx].3).pow(2) {
                            ret[circles[*idx].0] = circles[i].0;
                            idxes.remove(idx);
                        }
                    }
                }
            }
        }
    }

    for val in ret.iter().skip(1) {
        write!(out, "{val} ").unwrap();
    }

    writeln!(out).unwrap();
}
