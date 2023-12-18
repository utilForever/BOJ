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

fn calculate_bound_by_teachers(
    move_teachers: &Vec<Vec<(i64, i64)>>,
    turn: i64,
    n: usize,
    m: usize,
) -> Vec<Vec<bool>> {
    let mut bounds = vec![vec![false; m]; n];

    for moves in move_teachers.iter() {
        let pos = moves[turn as usize % moves.len()];

        for i in -1..=1 {
            for j in -1..=1 {
                let x_next = pos.0 + i;
                let y_next = pos.1 + j;

                if x_next < 0 || y_next < 0 || x_next >= n as i64 || y_next >= m as i64 {
                    continue;
                }

                bounds[x_next as usize][y_next as usize] = true;
            }
        }
    }

    bounds
}

fn process_bfs(
    map: &Vec<Vec<char>>,
    move_teachers: &Vec<Vec<(i64, i64)>>,
    time_limit: i64,
    time_eat: i64,
) -> bool {
    let dx: [i64; 9] = [-1, -1, -1, 0, 0, 0, 1, 1, 1];
    let dy: [i64; 9] = [-1, 0, 1, -1, 0, 1, -1, 0, 1];
    let pos_classroom = (0, 0);
    let pos_store = (map.len() - 1, map[0].len() - 1);

    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    let mut visited1 = vec![vec![false; map[0].len()]; map.len()];
    let mut visited2 = vec![vec![false; map[0].len()]; map.len()];
    let mut bound_teachers_curr =
        calculate_bound_by_teachers(move_teachers, 0, map.len(), map[0].len());
    let mut bound_teachers_next =
        calculate_bound_by_teachers(move_teachers, 0, map.len(), map[0].len());
    let mut is_arrived_store = false;

    queue1.push_back((pos_classroom.0, pos_classroom.1, -5));
    visited1[pos_classroom.0][pos_classroom.1] = true;

    let mut time_prev = -1;
    let mut turn = 0;

    // Step 1: Move from (0, 0) to (n - 1, m - 1)
    while !queue1.is_empty() {
        let (x, y, time) = queue1.pop_front().unwrap();

        if time_prev != time {
            time_prev = time;
            visited1 = vec![vec![false; map[0].len()]; map.len()];
            bound_teachers_curr = bound_teachers_next;
            bound_teachers_next =
                calculate_bound_by_teachers(move_teachers, turn + 1, map.len(), map[0].len());

            turn += 1;
        }

        if time + 10 > time_limit {
            continue;
        }

        for i in 0..9 {
            let x_next = x as i64 + dx[i];
            let y_next = y as i64 + dy[i];

            if x_next < 0
                || y_next < 0
                || x_next >= map.len() as i64
                || y_next >= map[0].len() as i64
            {
                continue;
            }

            let x_next = x_next as usize;
            let y_next = y_next as usize;

            if x_next == pos_store.0 && y_next == pos_store.1 {
                let time_next = ((time + time_eat + 4) / 10 * 10) + 5;
                turn = (time + time_eat + 4) / 10;

                queue2.push_back((x_next, y_next, time_next));
                visited2[x_next][y_next] = true;
                is_arrived_store = true;
                break;
            }

            if bound_teachers_curr[x_next][y_next] || bound_teachers_next[x_next][y_next] {
                if x == 0 && y == 0 && x_next == 0 && y_next == 0 {
                    queue1.push_back((x_next, y_next, time + 10));
                    visited1[x_next][y_next] = true;
                }

                continue;
            }

            if map[x_next][y_next] == '#' {
                continue;
            }

            if visited1[x_next][y_next] {
                continue;
            }

            queue1.push_back((x_next, y_next, time + 10));
            visited1[x_next][y_next] = true;
        }

        if is_arrived_store {
            break;
        }
    }

    bound_teachers_curr =
        calculate_bound_by_teachers(move_teachers, turn + 1, map.len(), map[0].len());

    // Step 2: Move from (n - 1, m - 1) to (0, 0)
    while !queue2.is_empty() {
        let (x, y, time) = queue2.pop_front().unwrap();

        if time_prev != time {
            time_prev = time;
            visited2 = vec![vec![false; map[0].len()]; map.len()];
            bound_teachers_curr = bound_teachers_next;
            bound_teachers_next =
                calculate_bound_by_teachers(move_teachers, turn + 1, map.len(), map[0].len());

            turn += 1;
        }

        if time + 10 > time_limit {
            continue;
        }

        for i in 0..9 {
            let x_next = x as i64 + dx[i];
            let y_next = y as i64 + dy[i];

            if x_next < 0
                || y_next < 0
                || x_next >= map.len() as i64
                || y_next >= map[0].len() as i64
            {
                continue;
            }

            let x_next = x_next as usize;
            let y_next = y_next as usize;

            if x_next == pos_classroom.0 && y_next == pos_classroom.1 {
                return true;
            }

            if bound_teachers_curr[x_next][y_next] || bound_teachers_next[x_next][y_next] {
                if x == map.len() - 1
                    && y == map[0].len() - 1
                    && x_next == map.len() - 1
                    && y_next == map[0].len() - 1
                {
                    queue2.push_back((x_next, y_next, time + 10));
                    visited2[x_next][y_next] = true;
                }
                
                continue;
            }

            if map[x_next][y_next] == '#' {
                continue;
            }

            if visited2[x_next][y_next] {
                continue;
            }

            queue2.push_back((x_next, y_next, time + 10));
            visited2[x_next][y_next] = true;
        }
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, l, t, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut map = vec![vec![' '; m]; n];
    let mut move_teachers = vec![Vec::new(); l];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            map[i][j] = c;
        }
    }

    for i in 0..l {
        let p = scan.token::<i64>();

        for _ in 0..p {
            move_teachers[i].push((scan.token::<i64>() - 1, scan.token::<i64>() - 1));
        }
    }

    if t < 20 * (n.max(m) as i64 - 1) + k - 15 {
        writeln!(out, "SAD").unwrap();
        return;
    }

    writeln!(
        out,
        "{}",
        if process_bfs(&map, &move_teachers, t, k) {
            "YUMMY"
        } else {
            "SAD"
        }
    )
    .unwrap();
}
