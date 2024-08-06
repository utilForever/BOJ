use io::Write;
use std::ops::Bound::*;
use std::{collections::BTreeSet, io, str};

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

fn insert(
    set: &mut BTreeSet<(usize, usize)>,
    ret: &mut BTreeSet<(i64, (i64, usize))>,
    skills: &Vec<i64>,
    left: usize,
    right: usize,
) {
    if left > right {
        return;
    }

    set.insert((left, right));
    ret.insert((-(skills[left] as i64), (-((right - left + 1) as i64), left)));
}

fn remove(
    set: &mut BTreeSet<(usize, usize)>,
    ret: &mut BTreeSet<(i64, (i64, usize))>,
    skills: &Vec<i64>,
    left: usize,
    right: usize,
) {
    if left > right {
        return;
    }

    set.remove(&(left, right));
    ret.remove(&(-(skills[left] as i64), (-((right - left + 1) as i64), left)));
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut skills = vec![0; n + 2];
    let mut set = BTreeSet::new();
    let mut ret = BTreeSet::new();

    skills[0] = 1 << 30;
    skills[n + 1] = 1 << 30;

    for i in 1..=n {
        skills[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        if !set.is_empty() && skills[i] == skills[i - 1] {
            let prev = *set.iter().rev().next().unwrap();

            remove(&mut set, &mut ret, &skills, prev.0, prev.1);
            insert(&mut set, &mut ret, &skills, prev.0, prev.1 + 1);
        } else {
            insert(&mut set, &mut ret, &skills, i, i);
        }
    }

    let q = scan.token::<i64>();

    for _ in 0..q {
        let (x, y) = (scan.token::<usize>(), scan.token::<i64>());
        let mut before = set.range(..=(x, 1 << 30));
        let prev = *before.next_back().unwrap();

        remove(&mut set, &mut ret, &skills, prev.0, prev.1);

        skills[x] = y;

        insert(&mut set, &mut ret, &skills, x, x);
        insert(&mut set, &mut ret, &skills, prev.0, x - 1);
        insert(&mut set, &mut ret, &skills, x + 1, prev.1);

        let mut next = set.range((Included((x, x)), Unbounded));
        let val = next.clone().next().unwrap();

        if next.clone().next().unwrap() != set.iter().next().unwrap() && y == skills[x - 1] {
            let left = *set.range(..=(x, x)).nth_back(1).unwrap();
            let value = *next.clone().next().unwrap();

            remove(&mut set, &mut ret, &skills, left.0, left.1);
            remove(&mut set, &mut ret, &skills, value.0, value.1);
            insert(&mut set, &mut ret, &skills, left.0, left.1 + 1);

            next = set.range((Included((left.0, left.1 + 1)), Unbounded));
        }

        if next.clone().nth(1).is_some() && y == skills[x + 1] {
            let right = *next.clone().nth(1).unwrap();
            let value = *next.clone().next().unwrap();

            remove(&mut set, &mut ret, &skills, value.0, value.1);
            remove(&mut set, &mut ret, &skills, right.0, right.1);
            insert(&mut set, &mut ret, &skills, value.0, right.1);
        }

        let val = ret.iter().next().unwrap().1;
        writeln!(out, "{} {}", val.1, val.1 - val.0 as usize - 1).unwrap();
    }
}
