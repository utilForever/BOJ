use io::Write;
use std::{collections::VecDeque, io, str};

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

    let (n, m, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut arrows = Vec::with_capacity(k);
    let mut grid = vec![vec![0; m + 1]; n + 1];
    let mut idxes_in_arrow = vec![vec![0; m + 1]; n + 1];

    for i in 0..k {
        let (mut r, mut c, l, s) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<usize>(),
            scan.token::<String>(),
        );

        let mut positions = Vec::with_capacity(l + 1);
        positions.push((r, c));

        for ch in s.chars() {
            match ch {
                'U' => r -= 1,
                'D' => r += 1,
                'L' => c -= 1,
                'R' => c += 1,
                _ => unreachable!(),
            }

            positions.push((r, c));
        }

        let len = positions.len();
        let direction = (
            positions[len - 1].0 - positions[len - 2].0,
            positions[len - 1].1 - positions[len - 2].1,
        );

        for (idx, &(r, c)) in positions.iter().enumerate() {
            grid[r as usize][c as usize] = i + 1;
            idxes_in_arrow[r as usize][c as usize] = idx;
        }

        arrows.push((positions, direction));
    }

    let mut rev = vec![Vec::new(); k];
    let mut need_arrows_remove = vec![0; k];

    for i in 0..k {
        let &(head_r, head_c) = arrows[i].0.last().unwrap();
        let mut r = head_r + arrows[i].1 .0;
        let mut c = head_c + arrows[i].1 .1;
        let mut step = 1;

        let mut dependencies = Vec::new();
        let mut check = false;

        while 1 <= r && r <= n as i64 && 1 <= c && c <= m as i64 {
            let r_next = r as usize;
            let c_next = c as usize;

            if grid[r_next][c_next] != 0 {
                if grid[r_next][c_next] == i + 1 {
                    let j = idxes_in_arrow[r_next][c_next];

                    if j >= step - 1 {
                        check = true;
                        break;
                    }
                } else {
                    dependencies.push(grid[r_next][c_next] - 1);
                }
            }

            r += arrows[i].1 .0;
            c += arrows[i].1 .1;
            step += 1;
        }

        if check {
            writeln!(out, "No").unwrap();
            return;
        }

        dependencies.sort_unstable();
        dependencies.dedup();

        need_arrows_remove[i] = dependencies.len();

        for dependency in dependencies {
            rev[dependency].push(i);
        }
    }

    let mut queue = VecDeque::new();

    for i in 0..k {
        if need_arrows_remove[i] == 0 {
            queue.push_back(i);
        }
    }

    let mut cnt_removed = 0;

    while let Some(x) = queue.pop_front() {
        cnt_removed += 1;

        for &y in rev[x].iter() {
            need_arrows_remove[y] -= 1;

            if need_arrows_remove[y] == 0 {
                queue.push_back(y);
            }
        }
    }

    writeln!(out, "{}", if cnt_removed == k { "Yes" } else { "No" }).unwrap();
}
