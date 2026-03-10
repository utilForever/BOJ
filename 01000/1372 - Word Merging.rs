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
    let mut strings = vec![String::new(); n];

    for i in 0..n {
        strings[i] = scan.token::<String>();
    }

    strings.sort_unstable();
    strings.dedup();

    let mut strings_filtered = Vec::new();

    for i in 0..strings.len() {
        let mut check = false;

        for j in 0..strings.len() {
            if i == j {
                continue;
            }

            if strings[j].contains(&strings[i]) {
                check = true;
                break;
            }
        }

        if !check {
            strings_filtered.push(strings[i].clone());
        }
    }

    let words = strings_filtered
        .into_iter()
        .map(|s| s.into_bytes())
        .collect::<Vec<_>>();
    let m = words.len();

    let mut overlap_len = vec![vec![0; m]; m];

    for i in 0..m {
        for j in 0..m {
            if i == j {
                continue;
            }

            for k in (0..=words[i].len().min(words[j].len())).rev() {
                if words[i][words[i].len() - k..] == words[j][..k] {
                    overlap_len[i][j] = k;
                    break;
                }
            }
        }
    }

    let mut dp = vec![None; m * (1 << m)];

    for i in 0..m {
        dp[m * (1 << i) + i] = Some(words[i].clone());
    }

    for mask in 1..1 << m {
        for i in 0..m {
            if (mask & (1 << i)) == 0 {
                continue;
            }

            if mask == 1 << i {
                continue;
            }

            let mask_prev = mask ^ (1 << i);
            let mut val = None;

            for j in 0..m {
                if (mask_prev & (1 << j)) == 0 {
                    continue;
                }

                if let Some(dp_prev) = &dp[m * mask_prev + j] {
                    let mut candidate = dp_prev.clone();
                    candidate.extend_from_slice(&words[i][overlap_len[j][i]..]);

                    match val {
                        None => val = Some(candidate),
                        Some(ref mut current) => {
                            if candidate.len() < current.len()
                                || (candidate.len() == current.len() && candidate < *current)
                            {
                                *current = candidate;
                            }
                        }
                    }
                }
            }

            dp[m * mask + i] = val;
        }
    }

    let mut ret = None;

    for i in 0..m {
        if let Some(candidate) = dp[m * ((1 << m) - 1) + i].clone() {
            match ret {
                None => ret = Some(candidate.clone()),
                Some(ref mut current) => {
                    if candidate.len() < current.len()
                        || (candidate.len() == current.len() && candidate < *current)
                    {
                        *current = candidate.clone();
                    }
                }
            }
        }
    }

    writeln!(out, "{}", String::from_utf8(ret.unwrap()).unwrap()).unwrap();
}
