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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut nums = vec![0; n];

    for i in 0..n {
        nums[i] = scan.token::<i64>();
    }

    let mut values_len_max = HashMap::new();
    let mut max_top_three = Vec::new();
    let mut ret = 0;

    for i in 0..n {
        let mut len_max_prev = 0i64;

        for &(value, len) in max_top_three.iter() {
            if value != nums[i] - 1 && value != nums[i] + 1 {
                len_max_prev = len_max_prev.max(len);
            }
        }

        let len_max_cand = len_max_prev + 1;
        let len_max_curr = *values_len_max.get(&nums[i]).unwrap_or(&0);

        if len_max_cand > len_max_curr {
            values_len_max.insert(nums[i], len_max_cand);

            if let Some(pos) = max_top_three
                .iter()
                .position(|&(value, _)| value == nums[i])
            {
                max_top_three.remove(pos);
            }

            max_top_three.push((nums[i], len_max_cand));
            max_top_three.sort_unstable_by(|a, b| b.1.cmp(&a.1));

            if max_top_three.len() > 3 {
                max_top_three.pop();
            }

            ret = ret.max(len_max_cand);
        } else {
            ret = ret.max(len_max_curr);
        }
    }

    writeln!(out, "{ret}").unwrap();
}
