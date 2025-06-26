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
    let mut people = vec![(0, 0); n];

    for i in 0..n {
        people[i] = (scan.token::<i64>(), scan.token::<i64>());
    }

    let mut start_max = people[0].0;
    let mut position = people[0].0;
    let mut leap = people[0].1;
    let mut possible = true;

    for i in 1..n {
        start_max = start_max.max(people[i].0);

        let g = gcd(leap, people[i].1);
        let multiplier = people[i].1 / g;
        let mut found = false;

        for j in 0..multiplier {
            if (position + leap * j) % people[i].1 == people[i].0 % people[i].1 {
                position += leap * j;
                leap *= multiplier;
                found = true;
                break;
            }
        }

        if !found {
            possible = false;
            break;
        }
    }

    if !possible {
        writeln!(out, "-1").unwrap();
        return;
    }

    if position < start_max {
        position += ((start_max - position + leap - 1) / leap) * leap;
    }

    writeln!(out, "{position}").unwrap();
}
