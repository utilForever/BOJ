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

fn process_cleaning(
    is_cleaned: &mut Vec<Vec<bool>>,
    visited: &mut Vec<Vec<Vec<i64>>>,
    rule_a: &Vec<Vec<u32>>,
    rule_b: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    turn: u32,
    cnt: usize,
    ret: &mut i64,
) {
    let dx: [i64; 4] = [-1, 0, 1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    let mut is_rule_a = false;

    if !is_cleaned[x][y] {
        is_rule_a = true;
        is_cleaned[x][y] = true;
    }

    let turn_next;

    if is_rule_a {
        *ret = cnt as i64;
        turn_next = (turn + rule_a[x][y]) % 4;
    } else {
        if visited[x][y][turn as usize] >= (rule_a.len() * rule_a[0].len()) as i64 {
            return;
        }

        visited[x][y][turn as usize] += 1;
        turn_next = (turn + rule_b[x][y]) % 4;
    }

    let x_next = x as i64 + dx[turn_next as usize];
    let y_next = y as i64 + dy[turn_next as usize];

    if x_next < 0 || x_next >= rule_a.len() as i64 || y_next < 0 || y_next >= rule_a[0].len() as i64
    {
        return;
    }

    process_cleaning(
        is_cleaned,
        visited,
        rule_a,
        rule_b,
        x_next as usize,
        y_next as usize,
        turn_next,
        cnt + 1,
        ret,
    );
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());
    let mut rule_a = vec![vec![0; w]; h];
    let mut rule_b = vec![vec![0; w]; h];

    let (r, c, d) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<u32>(),
    );

    for i in 0..h {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            rule_a[i][j] = c.to_digit(10).unwrap();
        }
    }

    for i in 0..h {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            rule_b[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut is_cleaned = vec![vec![false; w]; h];
    let mut visited = vec![vec![vec![0; 4]; w]; h];
    let mut ret = 0;

    process_cleaning(
        &mut is_cleaned,
        &mut visited,
        &rule_a,
        &rule_b,
        r,
        c,
        d,
        0,
        &mut ret,
    );

    writeln!(out, "{}", ret + 1).unwrap();
}
