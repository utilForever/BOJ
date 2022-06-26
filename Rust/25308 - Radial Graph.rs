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

fn is_convex(batched_abilities: &[i64; 8]) -> bool {
    for i in 0..8 {
        let p = i;
        let q = (i + 1) % 8;
        let r = (i + 2) % 8;
        if (batched_abilities[p] * batched_abilities[r]) as f64 * 2.0_f64.sqrt()
            > (batched_abilities[q] * (batched_abilities[p] + batched_abilities[r])) as f64
        {
            return false;
        }
    }

    true
}

fn process_dfs(
    abilities: &[i64; 8],
    batched_abilities: &mut [i64; 8],
    visited: &mut [bool; 8],
    ret: &mut i64,
    idx: usize,
) {
    if idx == 8 {
        if is_convex(batched_abilities) {
            *ret += 1;
        }

        return;
    }

    for i in 0..8 {
        if visited[i] {
            continue;
        }

        visited[i] = true;
        batched_abilities[idx] = abilities[i];
        process_dfs(abilities, batched_abilities, visited, ret, idx + 1);
        visited[i] = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut abilities = [0; 8];
    let mut batched_abilities = [0; 8];
    let mut visited = [false; 8];

    for i in 0..8 {
        abilities[i] = scan.token::<i64>();
    }

    let mut ret = 0;

    process_dfs(
        &abilities,
        &mut batched_abilities,
        &mut visited,
        &mut ret,
        0,
    );

    writeln!(out, "{ret}").unwrap();
}
