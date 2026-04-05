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

fn gcd(mut first: i128, mut second: i128) -> i128 {
    if first < 0 {
        first = -first;
    }

    if second < 0 {
        second = -second;
    }

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

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut portals = vec![(0, 0); n];

    for i in 0..n {
        portals[i] = (scan.token::<i128>(), scan.token::<i128>());
    }

    if n == 1 && portals[0] == (0, 0) {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut diffs = Vec::with_capacity(n - 1);

    for i in 1..n {
        diffs.push((portals[i].0 - portals[0].0, portals[i].1 - portals[0].1));
    }

    if diffs
        .iter()
        .skip(1)
        .all(|&diff| diff.0 * diffs[0].1 == diff.1 * diffs[0].0)
    {
        writeln!(out, "-1").unwrap();
        return;
    }

    let mut base = None;

    for &(x, y) in diffs.iter() {
        if x == 0 {
            continue;
        }

        base = if let Some((bx, by)) = base {
            let (_, s, t) = gcd_extended(bx, x);
            Some((s * bx + t * x, s * by + t * y))
        } else {
            Some((x, y))
        }
    }

    let mut base = base.unwrap();

    if base.0 < 0 {
        base = (-base.0, -base.1);
    }

    let mut step = 0;

    for &(x, y) in diffs.iter() {
        step = gcd(step, (y - (x / base.0) * base.1).abs());
    }

    writeln!(out, "{}", base.0 * step).unwrap();
}
