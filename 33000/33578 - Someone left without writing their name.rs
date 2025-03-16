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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

const INF: i64 = i64::MAX / 2;
const DY: [i64; 4] = [-1, 1, 0, 0];
const DX: [i64; 4] = [0, 0, -1, 1];

fn process_bfs(
    school: &Vec<Vec<char>>,
    dist: &mut Vec<Vec<i64>>,
    start: (usize, usize),
    n: usize,
    m: usize,
) {
    let mut queue = VecDeque::new();

    dist[start.0][start.1] = 0;
    queue.push_back(start);

    while let Some((i, j)) = queue.pop_front() {
        let dist_curr = dist[i][j];

        for k in 0..4 {
            let (i_next, j_next) = (i as i64 + DY[k], j as i64 + DX[k]);

            if i_next < 0 || i_next >= n as i64 || j_next < 0 || j_next >= m as i64 {
                continue;
            }

            let (i_next, j_next) = (i_next as usize, j_next as usize);

            if school[i_next][j_next] == '#' {
                continue;
            }

            if dist[i_next][j_next] > dist_curr + 1 {
                dist[i_next][j_next] = dist_curr + 1;
                queue.push_back((i_next, j_next));
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut school = vec![vec![' '; m]; n];
    let mut pos_seungchan = (0, 0);
    let mut pos_jinwoo = (0, 0);
    let mut teachers = Vec::new();

    for i in 0..n {
        let line = scan.line().trim().to_string();

        for (j, c) in line.chars().enumerate() {
            school[i][j] = c;

            match c {
                'S' => pos_seungchan = (i, j),
                'J' => pos_jinwoo = (i, j),
                'T' => teachers.push((i, j)),
                _ => {}
            }
        }
    }

    let mut dist_seungchan = vec![vec![INF; m]; n];
    let mut dist_jinwoo = vec![vec![INF; m]; n];

    process_bfs(&school, &mut dist_seungchan, pos_seungchan, n, m);
    process_bfs(&school, &mut dist_jinwoo, pos_jinwoo, n, m);

    let mut ret = INF;

    if dist_jinwoo[pos_seungchan.0][pos_seungchan.1] != INF {
        ret = dist_jinwoo[pos_seungchan.0][pos_seungchan.1] * 2;
    }

    for (i, j) in teachers {
        if dist_seungchan[i][j] != INF && dist_jinwoo[i][j] != INF {
            ret = ret.min(dist_seungchan[i][j] + dist_jinwoo[i][j] * 2);
        }
    }

    writeln!(out, "{}", if ret == INF { -1 } else { ret }).unwrap();
}
