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

    let (r, c) = (scan.token::<usize>(), scan.token::<usize>());
    let mut maze = vec![vec![' '; c]; r];
    let mut pos_jihun = (0, 0);

    for i in 0..r {
        let line = scan.token::<String>();

        for (j, c) in line.chars().enumerate() {
            maze[i][j] = c;

            if c == 'J' {
                pos_jihun = (i, j);
            }
        }
    }

    if pos_jihun.0 == 0 || pos_jihun.0 == r - 1 || pos_jihun.1 == 0 || pos_jihun.1 == c - 1 {
        writeln!(out, "1").unwrap();
        return;
    }

    let dy = [-1, 1, 0, 0];
    let dx = [0, 0, -1, 1];

    let mut deque_fire = VecDeque::new();
    let mut check_fire = vec![vec![-1; c]; r];

    for i in 0..r {
        for j in 0..c {
            if maze[i][j] == 'F' {
                deque_fire.push_back((i, j));
                check_fire[i][j] = 0;
            }
        }
    }

    while let Some((r_curr, c_curr)) = deque_fire.pop_front() {
        for i in 0..4 {
            let r_next = r_curr as i64 + dy[i];
            let c_next = c_curr as i64 + dx[i];

            if r_next < 0 || r_next >= r as i64 || c_next < 0 || c_next >= c as i64 {
                continue;
            }

            let r_next = r_next as usize;
            let c_next = c_next as usize;

            if maze[r_next][c_next] == '#' {
                continue;
            }

            if check_fire[r_next][c_next] == -1 {
                check_fire[r_next][c_next] = check_fire[r_curr][c_curr] + 1;
                deque_fire.push_back((r_next, c_next));
            }
        }
    }

    let mut deque_jihun = VecDeque::new();
    let mut check_jihun = vec![vec![-1; c]; r];
    let mut ret = -1;

    deque_jihun.push_back(pos_jihun);
    check_jihun[pos_jihun.0][pos_jihun.1] = 0;

    'outer: while let Some((r_curr, c_curr)) = deque_jihun.pop_front() {
        for i in 0..4 {
            let r_next = r_curr as i64 + dy[i];
            let c_next = c_curr as i64 + dx[i];

            if r_next < 0 || r_next >= r as i64 || c_next < 0 || c_next >= c as i64 {
                ret = check_jihun[r_curr][c_curr] + 1;
                break 'outer;
            }

            let r_next = r_next as usize;
            let c_next = c_next as usize;

            if maze[r_next][c_next] == '#' || check_jihun[r_next][c_next] != -1 {
                continue;
            }

            if check_fire[r_next][c_next] != -1
                && check_fire[r_next][c_next] <= check_jihun[r_curr][c_curr] + 1
            {
                continue;
            }

            check_jihun[r_next][c_next] = check_jihun[r_curr][c_curr] + 1;
            deque_jihun.push_back((r_next, c_next));
        }
    }

    if ret == -1 {
        writeln!(out, "IMPOSSIBLE").unwrap();
    } else {
        writeln!(out, "{ret}").unwrap();
    }
}
