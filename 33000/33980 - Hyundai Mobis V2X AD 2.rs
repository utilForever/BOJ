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

fn adjust(route: &mut Vec<i64>, first: i64, last: i64) -> i64 {
    let n = route.len();

    if route[0] != first && route[n - 1] != last && route[0] != route[n - 1] {
        route.swap(0, n - 1);
        return 1;
    }

    let mut ret = 0;

    if route[0] != first {
        for i in (0..n).rev() {
            if route[i] != first {
                continue;
            }

            route.swap(0, i);
            ret += 1;
            break;
        }
    }

    if route[n - 1] != last {
        for i in 0..n {
            if route[i] != last {
                continue;
            }

            route.swap(i, n - 1);
            ret += 1;
            break;
        }
    }

    ret
}

fn calculate(route1: &Vec<i64>, route2: &Vec<i64>) -> i64 {
    let n = route1.len();
    let mut route1 = route1.clone();
    let mut route2 = route2.clone();
    let mut ret = 0;

    ret += adjust(&mut route1, 1, 0);
    ret += adjust(&mut route2, 0, 1);

    let mut diff = 0;
    let mut diff_min = i64::MAX;

    for i in 0..n - 1 {
        diff += route1[i] - route2[i];
        diff_min = diff_min.min(diff);
    }

    ret - diff_min + 1
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let route1 = scan.token::<String>().chars().collect::<Vec<_>>();
    let route2 = scan.token::<String>().chars().collect::<Vec<_>>();
    let mut route1_converted = vec![0; 2 * n];
    let mut route2_converted = vec![0; 2 * n];

    for i in 0..2 * n {
        route1_converted[i] = if route1[i] == 'U' { 1 } else { 0 };
        route2_converted[i] = if route2[i] == 'U' { 1 } else { 0 };
    }

    let ret1 = calculate(&route1_converted, &route2_converted);
    let ret2 = calculate(&route2_converted, &route1_converted);

    writeln!(out, "{}", ret1.min(ret2)).unwrap();
}
