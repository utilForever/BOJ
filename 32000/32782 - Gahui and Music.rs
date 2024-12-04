use io::Write;
use std::{io, str};

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

    let n = scan.token::<usize>();
    let s = scan.line().trim().to_string();
    let s = s.split_whitespace().collect::<Vec<_>>();

    let mut satisfactions = vec![0; n];
    let mut positions_s = Vec::new();
    let mut positions_ds = Vec::new();
    let mut idx_bar = 0;
    let mut idx = 0;

    while idx_bar < n {
        match s[idx] {
            "S" => {
                if idx + 1 < s.len() && (s[idx + 1] == "or" || s[idx + 1] == "or") {
                    positions_s.push(idx_bar);
                    positions_ds.push(idx_bar);
                    idx += 3;
                } else {
                    positions_s.push(idx_bar);
                    idx += 1;
                }
            }
            "DS" => {
                if idx + 1 < s.len() && (s[idx + 1] == "or" || s[idx + 1] == "or") {
                    positions_s.push(idx_bar);
                    positions_ds.push(idx_bar);
                    idx += 3;
                } else {
                    positions_ds.push(idx_bar);
                    idx += 1;
                }
            }
            val => {
                satisfactions[idx_bar] = val.parse::<i64>().unwrap();
                idx += 1;
            }
        }

        idx_bar += 1;
    }

    let mut prefix_sum = vec![0i64; n + 1];

    for i in 0..n {
        prefix_sum[i + 1] = prefix_sum[i] + satisfactions[i];
    }

    positions_s.sort();
    positions_ds.sort();

    let mut idx = 0;
    let mut idx_s = 0;
    let mut idx_ds = 0;
    let mut prefix_sum_min = prefix_sum[0];
    let mut ret = 0;

    while idx < n {
        if idx_s < positions_s.len() && positions_s[idx_s] == idx {
            let pos_s = positions_s[idx_s];

            prefix_sum_min = prefix_sum_min.min(prefix_sum[pos_s]);
            idx_s += 1;
        }

        if idx_ds < positions_ds.len() && positions_ds[idx_ds] == idx {
            let pos_ds = positions_ds[idx_ds];
            let additional = prefix_sum[pos_ds + 1] - prefix_sum_min;

            ret = ret.max(additional);
            idx_ds += 1;
        }

        idx += 1;
    }

    writeln!(out, "{}", ret + prefix_sum[n]).unwrap();
}
