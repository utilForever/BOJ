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
        let val = max;

        max = min;
        min = val;
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
    let mut vectors = vec![(0, 0); n];

    for i in 0..n {
        vectors[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut map = HashMap::new();

    for i in 0..n {
        let (mut x, mut y) = (vectors[i].0, vectors[i].1);
        let gcd = gcd(x, y);

        if gcd != 0 {
            x /= gcd;
            y /= gcd;
        }

        if x < 0 {
            x = -x;
            y = -y;
        }

        map.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
    }

    let mut ret = 0;

    for elem in map.iter() {
        let (mut x, mut y) = *elem.0;

        if y < 0 {
            y = -y;
        } else if x < 0 || y > 0 {
            x = -x;
        }

        if x != 0 || y != 0 {
            ret += elem.1 * *map.get(&(y, x)).unwrap_or(&0);
        }
    }

    let cnt_zero = *map.get(&(0, 0)).unwrap_or(&0);

    writeln!(
        out,
        "{}",
        ret / 2 + n * cnt_zero - cnt_zero * (cnt_zero + 1) / 2
    )
    .unwrap();
}
