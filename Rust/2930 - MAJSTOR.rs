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

enum Result {
    Win,
    Draw,
    Lose,
}

fn rsp(p1: char, p2: char) -> Result {
    match (p1, p2) {
        ('R', 'S') | ('S', 'P') | ('P', 'R') => Result::Win,
        ('R', 'R') | ('S', 'S') | ('P', 'P') => Result::Draw,
        _ => Result::Lose,
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let r = scan.token::<usize>();
    let sven = scan.token::<String>().chars().collect::<Vec<_>>();
    let n = scan.token::<usize>();
    let mut friends = vec![Vec::new(); n];

    for i in 0..n {
        friends[i] = scan.token::<String>().chars().collect();
    }

    let mut ret1 = 0;
    let mut ret2 = 0;

    for i in 0..r {
        let mut val_max = 0;

        // For ret1
        for _ in 0..3 {
            let mut val = 0;

            for j in 0..n {
                match rsp(sven[i], friends[j][i]) {
                    Result::Win => val += 2,
                    Result::Draw => val += 1,
                    Result::Lose => (),
                }
            }

            val_max = val_max.max(val);
        }

        ret1 += val_max;

        let mut val_max = 0;

        for p1 in ['R', 'S', 'P'] {
            let mut val = 0;

            for j in 0..n {
                match rsp(p1, friends[j][i]) {
                    Result::Win => val += 2,
                    Result::Draw => val += 1,
                    Result::Lose => (),
                }
            }

            val_max = val_max.max(val);
        }

        ret2 += val_max;
    }

    writeln!(out, "{ret1}").unwrap();
    writeln!(out, "{ret2}").unwrap();
}
