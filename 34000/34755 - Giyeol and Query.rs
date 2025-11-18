use io::Write;
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    ops::Bound::{Excluded, Unbounded},
    str,
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

fn insert(ranks: &mut BTreeMap<i64, BTreeMap<i64, BTreeSet<usize>>>, r: i64, h: i64, id: usize) {
    let by_h = ranks.entry(r).or_insert_with(BTreeMap::new);
    let ids = by_h.entry(h).or_insert_with(BTreeSet::new);
    ids.insert(id);
}

fn remove(ranks: &mut BTreeMap<i64, BTreeMap<i64, BTreeSet<usize>>>, r: i64, h: i64, id: usize) {
    let mut removed_rank = false;

    if let Some(by_h) = ranks.get_mut(&r) {
        let mut removed_honor = false;

        if let Some(ids) = by_h.get_mut(&h) {
            ids.remove(&id);

            if ids.is_empty() {
                removed_honor = true;
            }
        }

        if removed_honor {
            by_h.remove(&h);
        }

        if by_h.is_empty() {
            removed_rank = true;
        }
    }

    if removed_rank {
        ranks.remove(&r);
    }
}

fn find(ranks: &BTreeMap<i64, BTreeMap<i64, BTreeSet<usize>>>, r: i64, h: i64) -> Option<usize> {
    if let Some(by_h) = ranks.get(&r) {
        if let Some((_, ids)) = by_h.range((Excluded(&h), Unbounded)).next() {
            return ids.iter().next_back().copied();
        }
    }

    if let Some((_, by_h)) = ranks.range((Excluded(&r), Unbounded)).next() {
        if let Some((_, ids)) = by_h.iter().next() {
            return ids.iter().next_back().copied();
        }
    }
    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut rank = vec![0; n + 1];
    let mut honor = vec![0; n + 1];

    for i in 1..=n {
        rank[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        honor[i] = scan.token::<i64>();
    }

    let mut ranks: BTreeMap<i64, BTreeMap<i64, BTreeSet<usize>>> = BTreeMap::new();

    for i in 1..=n {
        insert(&mut ranks, rank[i], honor[i], i);
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let cmd = scan.token::<i64>();

        if cmd == 1 {
            let (a, b, c) = (
                scan.token::<usize>(),
                scan.token::<i64>(),
                scan.token::<i64>(),
            );

            remove(&mut ranks, rank[a], honor[a], a);
            rank[a] = b;
            honor[a] = c;
            insert(&mut ranks, rank[a], honor[a], a);
        } else {
            let a = scan.token::<usize>();
            let senior = find(&ranks, rank[a], honor[a]);
            let give = honor[a] / 2;

            remove(&mut ranks, rank[a], honor[a], a);
            honor[a] = honor[a] - give;
            insert(&mut ranks, rank[a], honor[a], a);

            if let Some(id) = senior {
                remove(&mut ranks, rank[id], honor[id], id);
                honor[id] = honor[id] + give;
                insert(&mut ranks, rank[id], honor[id], id);

                writeln!(out, "{} {} {} {}", rank[a], honor[a], rank[id], honor[id]).unwrap();
            } else {
                writeln!(out, "{} {} -1 -1", rank[a], honor[a]).unwrap();
            }
        }
    }
}
