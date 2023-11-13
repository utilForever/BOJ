use io::Write;
use std::{collections::BTreeMap, io, str};

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

fn find(parent: &mut Vec<usize>, node: usize) -> usize {
    if parent[node] == 0 {
        node
    } else {
        parent[node] = find(parent, parent[node]);
        parent[node]
    }
}

fn process_union(parent: &mut Vec<usize>, weeds: &Vec<(i64, i64)>, mut a: usize, mut b: usize) {
    a = find(parent, a);
    b = find(parent, b);

    if a == b {
        return;
    }

    if weeds[a].0.abs() + weeds[a].1.abs() < weeds[b].0.abs() + weeds[b].1.abs() {
        std::mem::swap(&mut a, &mut b);
    }

    parent[b] += parent[a];
    parent[a] = b;
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut weeds = vec![(0, 0); n + 1];
    let mut parent = vec![0; n + 1];
    let mut maps: BTreeMap<(i64, i64), Vec<usize>> = BTreeMap::new();

    for i in 1..=n {
        weeds[i] = (scan.token::<i64>(), scan.token::<i64>());
        maps.entry(weeds[i]).or_insert(Vec::new()).push(i);
    }

    for i in 1..=n {
        let (x_curr, y_curr) = weeds[i];

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let (x_next, y_next) = (x_curr as i64 + dx, y_curr as i64 + dy);

                if !maps.contains_key(&(x_next, y_next)) {
                    continue;
                }

                process_union(&mut parent, &weeds, maps[&(x_next, y_next)][0], i);
            }
        }
    }

    let mut val_max = 0;
    let mut ret = 0;

    for i in 1..=n {
        if find(&mut parent, i) != i {
            continue;
        }

        let val = weeds[i].0.abs() + weeds[i].1.abs();

        val_max = val_max.max(val);
        ret += 2 * val;
    }

    writeln!(out, "{}", ret - val_max).unwrap();
}
