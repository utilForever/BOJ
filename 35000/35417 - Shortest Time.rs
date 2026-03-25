use io::Write;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap},
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

    let n = scan.token::<i64>();

    if n == 1 {
        writeln!(out, "0").unwrap();
        return;
    }

    let t = scan.token::<i64>();
    let mut left = None;
    let mut right = if n == 2 { None } else { Some(2) };
    let mut inner = BTreeMap::new();

    let mut priority_queue: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
    let mut alive = Vec::new();
    let mut key_of = Vec::new();
    let mut id_next = 0;

    for t in 1..=t {
        if right.is_none() {
            writeln!(out, "{t}").unwrap();
            return;
        }

        while let Some(&Reverse((expected, id))) = priority_queue.peek() {
            if expected >= t {
                break;
            }

            priority_queue.pop();

            if alive[id] {
                inner.remove(&key_of[id]);
                alive[id] = false;
            }
        }

        if let Some(bound_right) = left {
            if bound_right - t < 1 {
                left = None;
            }
        }

        if let Some(bound_left) = right {
            if bound_left + t >= n {
                right = None;
            }
        }

        let (mut l, mut r) = (scan.token::<i64>(), scan.token::<i64>().min(n - 1));

        if l > r {
            continue;
        }

        if let Some(bound_right) = left {
            if bound_right - t > l {
                left = None;
                l = 1;
                r = r.max(bound_right - t);
            }
        }

        if let Some((&k, &(bound_right, id))) = inner.range(..=l - t).next_back() {
            if bound_right - t > l {
                inner.remove(&k);
                alive[id] = false;
                l = k + t;
                r = r.max(bound_right - t);
            }
        }

        loop {
            let next = inner
                .range(l - t..)
                .next()
                .map(|(&k, &(bound_right, id))| (k, bound_right, id));

            match next {
                Some((k, bound_right, id)) if k + t <= r + 1 => {
                    inner.remove(&k);
                    alive[id] = false;
                    r = r.max(bound_right - t);
                }
                _ => break,
            }
        }

        if let Some(bound_left) = right {
            if bound_left + t < r {
                right = None;
                l = l.min(bound_left + t);
                r = n - 1;
            }
        }

        if l == 1 && r == n - 1 {
            writeln!(out, "-1").unwrap();
            return;
        }

        if l == 1 {
            left = Some(r + t);
        } else if r == n - 1 {
            right = Some(l - t);
        } else {
            let id = id_next;
            let bound_left = l - t;
            let bound_right = r + t;

            inner.insert(bound_left, (bound_right, id));
            priority_queue.push(Reverse(((bound_right - bound_left) / 2, id)));
            alive.push(true);
            key_of.push(bound_left);
            id_next += 1;
        }
    }

    writeln!(
        out,
        "{}",
        if right.is_none() {
            t + 1
        } else {
            n - right.unwrap() + 1
        }
    )
    .unwrap();
}
