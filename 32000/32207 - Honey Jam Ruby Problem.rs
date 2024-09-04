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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

fn process_dfs(
    rubies: &Vec<(i64, usize, usize)>,
    selected: &mut Vec<(i64, usize, usize)>,
    ret: &mut i64,
    depth_curr: usize,
    cnt_curr: i64,
    k: i64,
    limit: usize,
) {
    if depth_curr == limit {
        let mut is_satisfy = true;
        let mut sum = 0;

        'outer: for &(value, y1, x1) in selected.iter() {
            sum += value;

            for &(_, y2, x2) in selected.iter() {
                if (y1 as i64 - y2 as i64).abs() + (x1 as i64 - x2 as i64).abs() == 1 {
                    is_satisfy = false;
                    break 'outer;
                }
            }
        }

        if is_satisfy && sum > *ret {
            *ret = sum;
        }

        return;
    }

    if cnt_curr < k {
        selected.push(rubies[depth_curr]);

        process_dfs(
            rubies,
            selected,
            ret,
            depth_curr + 1,
            cnt_curr + 1,
            k,
            limit,
        );
        
        selected.pop();
    }

    process_dfs(rubies, selected, ret, depth_curr + 1, cnt_curr, k, limit);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut rubies = Vec::with_capacity(n * m);

    for i in 0..n {
        for j in 0..m {
            let value = scan.token::<i64>();
            rubies.push((value, i, j));
        }
    }

    rubies.sort_by(|a, b| b.0.cmp(&a.0));

    let limit = (n * m).min(21);
    let mut selected = Vec::with_capacity(limit);
    let mut ret = 0;

    process_dfs(&rubies, &mut selected, &mut ret, 0, 0, k, limit);

    writeln!(out, "{ret}").unwrap();
}
