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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut people = vec![0; 3];
    let mut tickets = vec![0; 3];

    for i in 0..3 {
        people[i] = scan.token::<i64>();
    }

    for i in 0..3 {
        tickets[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    for i in 0..3 {
        if people[i] >= tickets[i] {
            ret += tickets[i];
            people[i] -= tickets[i];
            tickets[i] = 0;
        } else {
            ret += people[i];
            tickets[i] -= people[i];
            people[i] = 0;
        }
    }

    loop {
        let mut tmp = ret;

        for i in 0..3 {
            tickets[(i + 1) % 3] += tickets[i] / 3;
            tickets[i] %= 3;

            if people[(i + 1) % 3] >= tickets[(i + 1) % 3] {
                tmp += tickets[(i + 1) % 3];
                people[(i + 1) % 3] -= tickets[(i + 1) % 3];
                tickets[(i + 1) % 3] = 0;
            } else {
                tmp += people[(i + 1) % 3];
                tickets[(i + 1) % 3] -= people[(i + 1) % 3];
                people[(i + 1) % 3] = 0;
            }
        }

        if tmp == ret {
            break;
        } else {
            ret = tmp;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
