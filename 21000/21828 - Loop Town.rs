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

struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, _) = (scan.token::<usize>(), scan.token::<i64>());
    let mut positions = vec![(0, 0); n];

    for i in 0..n {
        positions[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    positions.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    positions.insert(0, (0, 0));

    let mut pos_offices = (1..=n).map(|i| positions[i].1).collect::<Vec<_>>();
    pos_offices.sort_unstable();

    let mut rank_offices = HashMap::with_capacity(n + 1);

    for (i, &pos) in pos_offices.iter().enumerate() {
        rank_offices.insert(pos, i + 1);
    }

    let mut fenwick_tree = FenwickTree::new(n + 2);
    let mut rank_office_to_home = vec![0; n + 2];
    let mut rank_home_to_office = vec![0; n + 2];
    let mut cnt_wrap_diff = vec![0; 2 * n + 5];

    for i in 1..=n {
        let rank = *rank_offices.get(&positions[i].1).unwrap();
        rank_office_to_home[rank] = i;
    }

    let mut base_crossing = 0;
    let mut wrap_cnt = 0;
    let mut wrap_sum = 0;

    for i in (1..=n).rev() {
        let rank_home = rank_office_to_home[i];

        base_crossing += fenwick_tree.query(rank_home);
        fenwick_tree.update(rank_home, 1);

        rank_home_to_office[rank_home] = i;

        if rank_home > i {
            wrap_cnt += 1;
            wrap_sum += 2 * (rank_home - i) as i64;
            cnt_wrap_diff[rank_home - i] += 1;
        }
    }

    let mut ret = i64::MAX;

    for i in 1..=n {
        let val = base_crossing - wrap_sum + wrap_cnt * (n as i64 - wrap_cnt);
        ret = ret.min(val);

        wrap_sum -= 2 * wrap_cnt;
        wrap_cnt -= cnt_wrap_diff[i];
        base_crossing += n as i64 - 2 * rank_home_to_office[i] as i64 + 1;

        if rank_home_to_office[i] < n {
            wrap_cnt += 1;
            wrap_sum += 2 * (n as i64 - rank_home_to_office[i] as i64);

            let idx = n - rank_home_to_office[i] + i;
            cnt_wrap_diff[idx] += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
