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

#[derive(Clone)]
struct FenwickTree {
    n: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree {
            n,
            data: vec![0; n + 1],
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.data[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut ret = 0;

        while idx > 0 {
            ret += self.data[idx];
            idx -= idx & (!idx + 1);
        }

        ret
    }
}

#[derive(Clone)]
struct Info {
    bit: FenwickTree,
    best: i64,
    best_users: HashMap<i64, i64>,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut submissions = vec![None; 100];

    for _ in 0..n {
        let (problem_id, user_id, t) = (
            scan.token::<usize>() - 100,
            scan.token::<i64>(),
            (scan.token::<f64>() * 1000.0 + 0.5) as i64,
        );

        if submissions[problem_id].is_none() {
            submissions[problem_id] = Some(Info {
                bit: FenwickTree::new(10000),
                best: i64::MAX,
                best_users: HashMap::new(),
            });
        }

        let submission = submissions[problem_id].as_mut().unwrap();

        if let Some(&t_old) = submission.best_users.get(&user_id) {
            if t >= t_old {
                writeln!(out, "submission ignored").unwrap();
                continue;
            }

            let idx_old = t_old as usize + 1;
            submission.bit.update(idx_old, -1);
        }

        let rank = submission.bit.query(t as usize + 1);

        submission.bit.update(t as usize + 1, 1);
        submission.best = submission.best.min(t);
        submission.best_users.insert(user_id, t);

        writeln!(
            out,
            "{} {:04} {:.3} {:.3} {}",
            problem_id + 100,
            user_id,
            t as f64 / 1000.0,
            submission.best as f64 / 1000.0,
            rank + 1,
        )
        .unwrap();
    }
}
