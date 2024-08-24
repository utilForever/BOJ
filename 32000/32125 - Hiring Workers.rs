use io::Write;
use std::{
    collections::{BTreeMap, HashMap},
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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (n, k) = (scan.token::<usize>(), scan.token::<i64>());
        let mut types = vec![0; n];
        let mut efficiencies = vec![0; n];

        for i in 0..n {
            types[i] = scan.token::<i64>();
        }

        for i in 0..n {
            efficiencies[i] = scan.token::<i64>();
        }

        let mut count_a = 0;
        let mut count_b = 0;
        let mut sum_a = 0;
        let mut sum_b = 0;
        let mut ret = 0;

        let mut map: HashMap<i64, BTreeMap<i64, i64>> = HashMap::new();
        map.insert(0, BTreeMap::new());
        map.get_mut(&0).unwrap().insert(0, 1);

        for i in 0..n {
            if types[i] == 1 {
                count_a += 1;
                sum_a += efficiencies[i];
            } else {
                count_b += 1;
                sum_b += efficiencies[i];
            }

            let diff_count = count_a - count_b;
            let diff_sum = sum_a - sum_b;

            if map.contains_key(&diff_count) {
                let iter = map
                    .get(&diff_count)
                    .unwrap()
                    .range(diff_sum - k..=diff_sum + k);

                for (_, value) in iter {
                    ret += value;
                }
            }

            map.entry(diff_count).or_insert(BTreeMap::new());
            *map.get_mut(&diff_count)
                .unwrap()
                .entry(diff_sum)
                .or_insert(0) += 1;
        }

        writeln!(out, "{ret}").unwrap();
    }
}
