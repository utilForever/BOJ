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
    let mut time_min = (4.0f64 * n as f64).sqrt().ceil() as isize - 2;

    if n < 5 && n != 3 {
        time_min += 1;
    }

    let time_min = time_min as usize;
    let mut used = vec![false; n + 1];
    let mut order = vec![0; time_min + 1];

    let mut sum = 0;
    let mut idx = 1;
    let mut step = time_min as isize + 1;

    while step > 0 {
        sum += step;
        order[idx] = sum as usize;
        idx += 1;
        step -= 2;
    }

    let mut idx = time_min;

    while order[idx] == 0 {
        order[idx] = order[time_min - idx] + 1;
        idx -= 1;
    }

    order[time_min] = 1;

    if time_min % 2 == 0 {
        order[time_min / 2] -= 1;
    }

    if n == 3 {
        order[1] = 3;
    }

    let mut idx = 0;

    while order[idx] < n {
        used[order[idx]] = true;
        idx += 1;
    }

    order[idx] = n;
    used[n] = true;
    idx += 1;

    while idx <= time_min {
        if order[idx - 1] <= order[idx] {
            order[idx] = order[idx - 1] - 1;

            while used[order[idx]] {
                order[idx] -= 1;
            }

            used[order[idx]] = true;
        }

        idx += 1;
    }

    writeln!(out, "{time_min}").unwrap();

    for i in 1..=time_min {
        write!(out, "{} ", order[i]).unwrap();
    }

    writeln!(out).unwrap();
}
