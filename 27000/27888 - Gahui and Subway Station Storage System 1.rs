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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut stations = vec![String::new(); n];
    let mut features_to_station = HashMap::new();
    let mut idx_feature = HashMap::new();
    let mut cnt_features = vec![0; 1 << 9];

    for i in 0..n {
        stations[i] = scan.token::<String>();
    }

    let r = scan.token::<i64>();

    for _ in 0..r {
        let command = scan.token::<char>();

        if command == 'U' {
            let (station, features) = (scan.token::<String>(), scan.token::<String>());

            if let Some(val) = features_to_station.get(&station) {
                let mut idx = *val;

                while idx > 0 {
                    cnt_features[idx as usize] -= 1;
                    idx = (idx - 1) & val;
                }
            }

            let mut val = 0;

            for feature in features.split(',') {
                let num_features = idx_feature.len();
                let index = *idx_feature
                    .entry(feature.to_string())
                    .or_insert(num_features);
                val |= 1 << index;
            }

            features_to_station.insert(station, val);

            let mut idx = val;

            while idx > 0 {
                cnt_features[idx as usize] += 1;
                idx = (idx - 1) & val;
            }
        } else {
            let features = scan.token::<String>();
            let mut val = 0;

            for feature in features.split(',') {
                if let Some(index) = idx_feature.get(feature) {
                    val |= 1 << index;
                } else {
                    val = 0;
                    break;
                }
            }

            writeln!(out, "{}", cnt_features[val as usize]).unwrap();
        }
    }
}
