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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut freq = vec![0; 5001];

    for _ in 0..n {
        let idx = scan.token::<usize>();
        freq[idx] += 1;
    }

    let mut vals = Vec::new();

    for i in 1..=5000 {
        if freq[i] > 0 {
            vals.push(i);
        }
    }

    let mut counts = vec![0; 5001];

    for i in 1..=5000 {
        let mut cnt = 0;
        let mut idx = i;

        while idx <= 5000 {
            cnt += freq[idx];
            idx += i;
        }

        counts[i] = cnt;
    }

    let mut gcd_a_multiple = vec![0; 5001];
    let mut gcd_a_not_multiple = vec![0; 5001];

    for i in 1..=5000 {
        let mut g = 0;
        let mut idx = i;

        while idx <= 5000 {
            if freq[idx] > 0 {
                g = if g == 0 {
                    idx as i64
                } else {
                    gcd(g as i64, idx as i64)
                }
            }

            idx += i;
        }

        gcd_a_multiple[i] = g;
    }

    for i in 1..=5000 {
        let mut g = 0;

        for &val in vals.iter() {
            if val % i == 0 {
                continue;
            }

            g = if g == 0 {
                val as i64
            } else {
                gcd(g as i64, val as i64)
            };
        }

        gcd_a_not_multiple[i] = g;
    }

    let mut gcd_a_div_multiple = vec![0; 5001];

    for i in 1..=5000 {
        let mut g = 0;
        let mut idx = 2 * i;

        while idx <= 5000 {
            if freq[idx] > 0 {
                g = if g == 0 {
                    (idx / i) as i64
                } else {
                    gcd(g as i64, (idx / i) as i64)
                }
            }

            idx += i;
        }

        gcd_a_div_multiple[i] = g;
    }

    let mut ret = 0;

    for i in 1..=5000 {
        if gcd_a_multiple[i] != i as i64 {
            continue;
        }

        ret = ret.max(gcd_a_not_multiple[i] + i as i64);

        if counts[i] != 1 && counts[i] == n {
            if freq[i] >= 2 || (freq[i] == 1 && gcd_a_div_multiple[i] == 1) {
                ret = ret.max(2 * i as i64);
            }
        }
    }

    writeln!(out, "{ret}").unwrap();
}
