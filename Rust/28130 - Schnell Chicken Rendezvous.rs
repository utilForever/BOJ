use io::Write;
use std::{collections::{BTreeMap, VecDeque}, io, str};

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

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut ground = vec![vec![' '; m]; n];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            ground[i][j] = c;

            if c == 'A' {
                start = (i, j);
            } else if c == 'B' {
                end = (i, j);
            }
        }
    }

    if ((start.0 as i64 - end.0 as i64).abs() + (start.1 as i64 - end.1 as i64).abs()) % 2 == 1 {
        writeln!(out, "-1").unwrap();
        return;
    }

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut dists = vec![vec![i64::MAX; m]; n];
    dists[start.0][start.1] = 0;

    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();

        for i in 0..4 {
            let (nx, ny) = (x as i64 + dx[i], y as i64 + dy[i]);

            if nx < 0 || nx >= n as i64 || ny < 0 || ny >= m as i64 {
                continue;
            }

            if ground[nx as usize][ny as usize] == 'G' {
                continue;
            }

            if dists[nx as usize][ny as usize] != i64::MAX {
                continue;
            }

            dists[nx as usize][ny as usize] = dists[x][y] + 1;
            queue.push_back((nx as usize, ny as usize));
        }
    }

    // Calculate track index of 'B'
    let mut len_track = 0;
    let mut map = BTreeMap::new();
    let mut curr = (end.0, end.1);

    loop {
        map.insert(curr, len_track);
        len_track += 1;

        if curr.0 == 0 && curr.1 < m - 1 {
            curr.1 += 1;
        } else if curr.1 == m - 1 && curr.0 < n - 1 {
            curr.0 += 1;
        } else if curr.0 == n - 1 && curr.1 > 0 {
            curr.1 -= 1;
        } else {
            curr.0 -= 1;
        }

        if curr == end {
            break;
        }
    }

    let mut curr = (end.0, end.1);
    let mut ret = i64::MAX;

    loop {
        if curr.0 == 0 && curr.1 < m - 1 {
            curr.1 += 1;
        } else if curr.1 == m - 1 && curr.0 < n - 1 {
            curr.0 += 1;
        } else if curr.0 == n - 1 && curr.1 > 0 {
            curr.1 -= 1;
        } else {
            curr.0 -= 1;
        }

        if curr == end {
            break;
        }

        let dist = dists[curr.0][curr.1];
        if dist == i64::MAX {
            continue;
        }

        let idx_start = map[&curr];
        let idx_end = dist % len_track;

        if idx_start == idx_end {
            ret = ret.min(dist);
        } else if idx_start < idx_end {
            ret = ret.min(dist + (len_track - (idx_end - idx_start)) as i64 / 2);
        } else {
            ret = ret.min(dist + (idx_start - idx_end) as i64 / 2);
        }
    }

    if ret == i64::MAX {
        writeln!(out, "-1").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}
