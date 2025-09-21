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

const SCALE: i64 = 1_000_000_000;

fn parse_fixed(s: &str) -> i64 {
    let mut iter = s.split('.');
    let part_integer = iter.next().unwrap().parse::<i64>().unwrap();
    let part_fraction = iter.next().unwrap_or("0");

    let mut val_fraction = 0;
    let mut cnt = 0;

    for c in part_fraction.chars() {
        if !c.is_ascii_digit() {
            break;
        }

        if cnt == 9 {
            break;
        }

        val_fraction = val_fraction * 10 + (c as u8 - b'0') as i64;
        cnt += 1;
    }

    for _ in cnt..9 {
        val_fraction *= 10;
    }

    part_integer * SCALE + val_fraction
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (l, c, s) = (
        scan.token::<i64>(),
        scan.token::<String>(),
        scan.token::<i64>(),
    );
    let (e, m) = (scan.token::<i64>(), scan.token::<i64>());
    let (c_max, c_min) = (scan.token::<String>(), scan.token::<String>());

    let c = parse_fixed(&c);
    let (c_max, c_min) = (parse_fixed(&c_max), parse_fixed(&c_min));

    let n = (s / m) as usize;
    let mut l_base = vec![0; n + 1];
    let mut c_base = vec![0; n + 1];

    l_base[0] = l;
    c_base[0] = c;

    for i in 0..n {
        l_base[i + 1] = l_base[i] + m;
        c_base[i + 1] = (c_base[i] * l_base[i]) / (l_base[i] + m);
    }

    if c_base[n] > c_max {
        writeln!(out, "0").unwrap();
        return;
    }

    if c_base[n] >= c_min && c_base[n] <= c_max {
        writeln!(out, "{}", l_base[n]).unwrap();
        return;
    }

    let mut left = 0;
    let mut right = n;

    while left <= n && c_base[left] > c_max {
        left += 1;
    }

    while right > 0 && c_base[right] < c_min {
        right -= 1;
    }

    if left > n || left > right {
        writeln!(out, "0").unwrap();
        return;
    }

    let mut ret = 0;

    for i in left..=right {
        let mut sum = 0;
        let mut l_curr = l_base[i];
        let mut c_curr = c_base[i];

        for _ in i..n {
            if c_curr < c_min {
                break;
            }

            if c_curr <= c_max && l_curr > 0 {
                let val = l_curr.min(e);

                l_curr -= val;
                sum += val;
            }

            c_curr = (c_curr * l_curr) / (l_curr + m);
            l_curr += m;
        }

        if c_curr >= c_min && c_curr <= c_max {
            sum += l_curr;
        }

        ret = ret.max(sum);
    }

    writeln!(out, "{ret}").unwrap();
}
