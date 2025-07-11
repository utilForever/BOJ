use io::Write;
use std::{collections::VecDeque, io, str};

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
    let mut disks = vec![0; n];

    for i in 0..n {
        disks[i] = scan.token::<i64>();
    }

    disks.sort_unstable();

    let mut deque = disks.into_iter().map(|x| x as f64).collect::<VecDeque<_>>();
    let mut v1 = Vec::with_capacity(n);
    let mut v2 = Vec::with_capacity(n);

    loop {
        if let Some(x) = deque.pop_back() {
            v1.push(x);
        } else {
            break;
        }

        if let Some(x) = deque.pop_front() {
            v2.insert(0, x);
        } else {
            break;
        }

        if let Some(x) = deque.pop_front() {
            v1.push(x);
        } else {
            break;
        }

        if let Some(x) = deque.pop_back() {
            v2.insert(0, x);
        } else {
            break;
        }
    }

    let mut v = v1;
    v.extend(v2);

    let n = v.len();
    let mut ret = f64::MAX;

    for i in 0..n {
        let mut w = v[i] + v[(i + n - 1) % n];

        for j in 1..n {
            let a = v[(i + j - 1) % n];
            let b = v[(i + j) % n];

            w += 2.0 * (a * b).sqrt();
        }

        ret = ret.min(w);
    }

    writeln!(out, "{:.12}", ret).unwrap();
}
