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

fn try_exit_maze(
    maze: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<i64>>,
    n: i64,
    m: i64,
    i: i64,
    j: i64,
) -> i64 {
    if i < 0 || i >= n || j < 0 || j >= m {
        return 1;
    }

    if visited[i as usize][j as usize] != -1 {
        return visited[i as usize][j as usize];
    }

    visited[i as usize][j as usize] = 0;

    match maze[i as usize][j as usize] {
        'U' => visited[i as usize][j as usize] = try_exit_maze(maze, visited, n, m, i - 1, j),
        'R' => visited[i as usize][j as usize] = try_exit_maze(maze, visited, n, m, i, j + 1),
        'D' => visited[i as usize][j as usize] = try_exit_maze(maze, visited, n, m, i + 1, j),
        'L' => visited[i as usize][j as usize] = try_exit_maze(maze, visited, n, m, i, j - 1),
        _ => unreachable!(),
    }

    visited[i as usize][j as usize]
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut maze = vec![vec!['0'; m]; n];
    let mut visited = vec![vec![-1; m]; n];

    for i in 0..n {
        let s = scan.token::<String>();

        for (j, c) in s.chars().enumerate() {
            maze[i][j] = c;
        }
    }

    let mut ret = 0;

    for i in 0..n {
        for j in 0..m {
            ret += try_exit_maze(&maze, &mut visited, n as i64, m as i64, i as i64, j as i64);
        }
    }

    writeln!(out, "{}", ret).unwrap();
}
