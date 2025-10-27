use io::Write;
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

const INF: i64 = i64::MAX / 4;

struct UnionFind {
    parent: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind { parent: vec![0; n] }
    }

    fn init(&mut self) {
        for i in 0..self.parent.len() {
            self.parent[i] = i;
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut a = x;

        while self.parent[a] != a {
            let a_parent = self.parent[a];
            let a_grandparent = self.parent[a_parent];
            self.parent[a] = a_grandparent;
            a = a_parent;
        }

        let root = a;
        let mut b = x;

        while self.parent[b] != root {
            let b_parent = self.parent[b];
            self.parent[b] = root;
            b = b_parent;
        }

        root
    }

    fn erase(&mut self, x: usize) {
        let root = self.find(x);
        let next = self.find(x + 1);
        self.parent[root] = next;
    }
}

struct Bucket {
    start: usize,
    end: usize,
    abs_min: i64,
}

impl Bucket {
    fn new(start: usize, end: usize, abs_min: i64) -> Self {
        Self {
            start,
            end,
            abs_min,
        }
    }

    fn rebuild(&mut self, alive: &Vec<bool>, finish: &Vec<i64>) {
        let mut finish_min = INF;

        for i in self.start..=self.end {
            if alive[i] {
                finish_min = finish_min.min(finish[i]);
            }
        }

        self.abs_min = finish_min;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut a = vec![0; n];
    let mut r = vec![0; n];
    let mut v = vec![0; n];

    for i in 0..n {
        a[i] = scan.token::<i64>();
        r[i] = scan.token::<i64>();
        v[i] = scan.token::<i64>();
    }

    let mut union_find = UnionFind::new(n + 1);
    union_find.init();

    let mut alive = vec![true; n];
    let mut speed = vec![1; n];
    let mut slack = vec![0; n];
    let mut finish = vec![0; n];

    for i in 0..n {
        finish[i] = a[i];
    }

    let bucket_len = ((n as f64).sqrt() as usize).max(512);
    let bucket_cnt = (n + bucket_len - 1) / bucket_len;
    let mut buckets = Vec::with_capacity(bucket_cnt);

    for b in 0..bucket_cnt {
        let start = b * bucket_len;
        let end = ((b + 1) * bucket_len).min(n) - 1;

        buckets.push(Bucket::new(start, end, 0));
    }

    let mut queue = BTreeSet::new();

    for (idx, bucket) in buckets.iter_mut().enumerate() {
        bucket.rebuild(&alive, &finish);
        queue.insert((bucket.abs_min, idx));
    }

    let mut done = 0;
    let mut time = 0;

    let mut touched_flag = vec![false; bucket_cnt];
    let mut touched_list = Vec::new();

    while done < n {
        let &(val, _) = queue.iter().next().unwrap();

        if val >= INF / 2 {
            break;
        }

        time = val;

        let mut buckets_today = Vec::new();

        loop {
            if let Some(&(val, idx)) = queue.iter().next() {
                if val == time {
                    queue.take(&(val, idx));
                    buckets_today.push(idx);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let mut finished = Vec::with_capacity(128);

        for &bucket in buckets_today.iter() {
            let (start, end) = (buckets[bucket].start, buckets[bucket].end);
            let mut abs_min = INF;

            for i in start..=end {
                if alive[i] && finish[i] == time {
                    alive[i] = false;
                    done += 1;
                    union_find.erase(i);
                    finished.push(i);
                } else if alive[i] {
                    abs_min = abs_min.min(finish[i]);
                }
            }

            buckets[bucket].abs_min = abs_min;
        }

        let mut events = Vec::with_capacity(finished.len() * 2);

        for &idx in finished.iter() {
            if v[idx] == 0 {
                continue;
            }

            let left = (idx as i64 - r[idx]).max(0) as usize;
            let right = (idx as i64 + r[idx]).min((n - 1) as i64) as usize;

            events.push((left, v[idx]));

            if right + 1 < n {
                events.push((right + 1, -v[idx]));
            }
        }

        if events.is_empty() {
            for &bucket in buckets_today.iter() {
                queue.insert((buckets[bucket].abs_min, bucket));
            }

            continue;
        }

        events.sort_unstable_by_key(|e| e.0);

        let mut segments = Vec::new();
        let mut curr = 0;
        let mut prev = 0;
        let mut event_idx = 0;

        while event_idx < events.len() {
            let pos = events[event_idx].0;

            if curr != 0 && prev < pos {
                segments.push((prev, pos - 1, curr));
            }

            while event_idx < events.len() && events[event_idx].0 == pos {
                curr += events[event_idx].1;
                event_idx += 1;
            }

            prev = pos;
        }

        if curr != 0 && prev < n {
            segments.push((prev, n - 1, curr));
        }

        for (l, r, delta) in segments {
            if delta == 0 {
                continue;
            }

            let mut idx = union_find.find(l);

            while idx <= r {
                if !alive[idx] {
                    idx = union_find.find(idx + 1);
                    continue;
                }

                let rem = speed[idx] * (finish[idx] - time) - slack[idx];
                let speed_new = speed[idx] + delta;
                let k_new = if speed_new <= 0 {
                    0
                } else {
                    (rem + speed_new - 1) / speed_new
                };
                let slack_new = speed_new * k_new - rem;
                let finish_new = time + k_new;

                speed[idx] = speed_new;
                slack[idx] = slack_new;
                finish[idx] = finish_new;

                let bucket = idx / bucket_len;

                if !touched_flag[bucket] {
                    touched_flag[bucket] = true;
                    touched_list.push(bucket);
                }

                idx = union_find.find(idx + 1);
            }
        }

        let mut need = Vec::with_capacity(buckets_today.len() + touched_list.len());
        need.extend_from_slice(&buckets_today);
        need.extend_from_slice(&touched_list);

        touched_list.clear();

        for &bucket in need.iter() {
            touched_flag[bucket] = false;
        }

        need.sort_unstable();
        need.dedup();

        for &bucket in need.iter() {
            if !buckets_today.binary_search(&bucket).is_ok() {
                queue.remove(&(buckets[bucket].abs_min, bucket));
            }

            buckets[bucket].rebuild(&alive, &finish);
            queue.insert((buckets[bucket].abs_min, bucket));
        }
    }

    writeln!(out, "{time}").unwrap();
}
