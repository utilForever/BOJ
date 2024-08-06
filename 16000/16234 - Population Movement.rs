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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, l, r) = (
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut nations = vec![vec![0; n]; n];
    let mut ret = 0;

    for i in 0..n {
        for j in 0..n {
            nations[i][j] = scan.token::<i64>();
        }
    }

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    loop {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; n]; n];
        let mut check_open = vec![vec![0; n]; n];
        let mut num_union = 0;

        // Step 1: Check the population movement is possible
        for i in 0..n {
            for j in 0..n {
                if visited[i][j] {
                    continue;
                }

                num_union += 1;

                queue.push_back((i, j));
                visited[i][j] = true;

                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();

                    for k in 0..4 {
                        let x_next = x as i64 + dx[k];
                        let y_next = y as i64 + dy[k];

                        if x_next < 0 || x_next >= n as i64 || y_next < 0 || y_next >= n as i64 {
                            continue;
                        }

                        let x_next = x_next as usize;
                        let y_next = y_next as usize;

                        if visited[x_next][y_next] {
                            continue;
                        }

                        let diff = (nations[x][y] - nations[x_next][y_next]).abs();

                        if diff < l || diff > r {
                            continue;
                        }

                        queue.push_back((x_next, y_next));
                        visited[x_next][y_next] = true;
                        check_open[x][y] = num_union;
                        check_open[x_next][y_next] = num_union;
                    }
                }
            }
        }

        if check_open.iter().all(|row| row.iter().all(|&x| x == 0)) {
            break;
        }

        // Step 2: Move the population
        let mut visited = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                if check_open[i][j] == 0 || visited[i][j] {
                    continue;
                }

                let mut queue = VecDeque::new();
                let mut list_to_move = Vec::new();
                let num_area = check_open[i][j];
                let mut cnt_area = 1;
                let mut num_population = nations[i][j];

                queue.push_back((i, j));
                list_to_move.push((i, j));
                visited[i][j] = true;

                while !queue.is_empty() {
                    let (x, y) = queue.pop_front().unwrap();

                    for k in 0..4 {
                        let x_next = x as i64 + dx[k];
                        let y_next = y as i64 + dy[k];

                        if x_next < 0 || x_next >= n as i64 || y_next < 0 || y_next >= n as i64 {
                            continue;
                        }

                        let x_next = x_next as usize;
                        let y_next = y_next as usize;

                        if visited[x_next][y_next] || check_open[x_next][y_next] != num_area {
                            continue;
                        }

                        queue.push_back((x_next, y_next));
                        list_to_move.push((x_next, y_next));
                        visited[x_next][y_next] = true;
                        cnt_area += 1;
                        num_population += nations[x_next][y_next];
                    }
                }

                let population_new = num_population / cnt_area;

                for (x, y) in list_to_move {
                    nations[x][y] = population_new;
                }
            }
        }

        ret += 1;
    }

    writeln!(out, "{ret}").unwrap();
}
