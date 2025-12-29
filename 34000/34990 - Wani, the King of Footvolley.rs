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

#[derive(Clone, Copy)]
struct Member {
    x: i64,
    y: i64,
    r: i64,
    skill: i64,
}

impl Member {
    fn new(x: i64, y: i64, r: i64, skill: i64) -> Self {
        Self { x, y, r, skill }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<i64>());
    let mut members = Vec::with_capacity(n);

    for _ in 0..n {
        let (x, y, r, skill) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        members.push(Member::new(x, y, r, skill));
    }

    let mut skip_next = false;
    let mut ret = 0;

    for _ in 0..m {
        let (tx, ty) = (scan.token::<i64>(), scan.token::<i64>());

        if skip_next {
            skip_next = false;
            continue;
        }

        let mut idx_max = -1;
        let mut skill_max = -1;

        for (idx, member) in members.iter().enumerate() {
            let dist = (member.x - tx).pow(2) + (member.y - ty).pow(2);

            if dist <= member.r.pow(2) && member.skill > skill_max {
                skill_max = member.skill;
                idx_max = idx as i64;
            }
        }

        if idx_max == 0 {
            ret += 1;
        } else if idx_max == -1 {
            skip_next = true;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
