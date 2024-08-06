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

fn process_dfs(
    candidates: &Vec<Vec<usize>>,
    check: &mut Vec<bool>,
    assigned_couples: &mut Vec<usize>,
    n: usize,
) -> bool {
    for i in 0..candidates[n].len() {
        let candidate = candidates[n][i];

        if check[candidate] {
            continue;
        }

        check[candidate] = true;

        if assigned_couples[candidate] == 0
            || process_dfs(
                candidates,
                check,
                assigned_couples,
                assigned_couples[candidate],
            )
        {
            assigned_couples[candidate] = n;
            return true;
        }
    }

    false
}
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut height_womans = vec![0; n + 1];
    let mut height_mans = vec![0; m + 1];
    let mut standard_womans = vec![0; n + 1];
    let mut standard_mans = vec![0; m + 1];

    for i in 1..=n {
        height_womans[i] = scan.token::<i64>();
    }

    for i in 1..=m {
        height_mans[i] = scan.token::<i64>();
    }

    for i in 1..=n {
        standard_womans[i] = scan.token::<i64>();
    }

    for i in 1..=m {
        standard_mans[i] = scan.token::<i64>();
    }

    let mut candidates = vec![Vec::new(); n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if height_womans[i] > standard_mans[j] && height_mans[j] < standard_womans[i] {
                candidates[i].push(j);
            }
        }
    }

    let mut check = vec![false; 401];
    let mut assigned_couple = vec![0; 401];
    let mut ret = 0;

    for i in 1..=n {
        check.fill(false);

        if process_dfs(&candidates, &mut check, &mut assigned_couple, i) {
            ret += 1;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
