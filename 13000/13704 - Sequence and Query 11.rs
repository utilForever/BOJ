use io::Write;
use std::{cmp::Ordering, io, str};

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

struct Query {
    l: usize,
    r: usize,
    idx: usize,
}

static mut BLOCK_SIZE: usize = 0;

fn cmp_mos(a: &Query, b: &Query) -> Ordering {
    unsafe {
        let block_a = a.l / BLOCK_SIZE;
        let block_b = b.l / BLOCK_SIZE;

        if block_a != block_b {
            return block_a.cmp(&block_b);
        }

        a.r.cmp(&b.r)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
    let mut arr = vec![0; n];

    for i in 0..n {
        arr[i] = scan.token::<i64>();
    }

    let m = scan.token::<usize>();
    let mut queries = Vec::with_capacity(m);

    for i in 0..m {
        let (l, r) = (scan.token::<usize>() - 1, scan.token::<usize>());
        queries.push(Query { l, r, idx: i });
    }

    let mut prefix_xor = vec![0; n + 1];

    for i in 1..=n {
        prefix_xor[i] = prefix_xor[i - 1] ^ arr[i - 1];
    }

    unsafe {
        BLOCK_SIZE = (n as f64).sqrt().floor() as usize;
    }

    queries.sort_by(cmp_mos);

    let add = |pos: usize, freq: &mut [i64], ret: &mut i64| {
        let val = prefix_xor[pos];
        let want = val ^ k;

        *ret += freq[want as usize];
        freq[val as usize] += 1;
    };

    let remove = |pos: usize, freq: &mut [i64], ret: &mut i64| {
        let val = prefix_xor[pos];
        let want = val ^ k;

        freq[val as usize] -= 1;
        *ret -= freq[want as usize];
    };

    let mut left = queries[0].l as i64;
    let mut right = left as i64 - 1;
    let mut freq = vec![0; 1 << 21];
    let mut val = 0;
    let mut ret = vec![0; m];

    for query in queries.iter() {
        let query_left = query.l;
        let query_right = query.r;

        while left > query_left as i64 {
            left -= 1;
            add(left as usize, &mut freq, &mut val);
        }

        while left < query_left as i64 {
            remove(left as usize, &mut freq, &mut val);
            left += 1;
        }

        while right < query_right as i64 {
            right += 1;
            add(right as usize, &mut freq, &mut val);
        }

        while right > query_right as i64 {
            remove(right as usize, &mut freq, &mut val);
            right -= 1;
        }

        ret[query.idx] = val;
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
