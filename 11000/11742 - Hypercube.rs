use io::Write;
use std::{collections::HashSet, io, str};

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
    cube: &Vec<Vec<Vec<char>>>,
    visited: &mut Vec<Vec<Vec<bool>>>,
    set: &mut HashSet<i64>,
    x: usize,
    y: usize,
    z: usize,
    a: i64,
    b: i64,
    c: i64,
    d: i64,
) {
    if visited[x][y][z] {
        return;
    }

    visited[x][y][z] = true;
    set.insert(d);

    if cube[x + 1][y][z] == 'x' {
        process_dfs(cube, visited, set, x + 1, y, z, d, b, c, -a);
    }

    if cube[x - 1][y][z] == 'x' {
        process_dfs(cube, visited, set, x - 1, y, z, -d, b, c, a);
    }

    if cube[x][y + 1][z] == 'x' {
        process_dfs(cube, visited, set, x, y + 1, z, a, d, c, -b);
    }

    if cube[x][y - 1][z] == 'x' {
        process_dfs(cube, visited, set, x, y - 1, z, a, -d, c, b);
    }

    if cube[x][y][z + 1] == 'x' {
        process_dfs(cube, visited, set, x, y, z + 1, a, b, d, -c);
    }

    if cube[x][y][z - 1] == 'x' {
        process_dfs(cube, visited, set, x, y, z - 1, a, b, -d, c);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (m, n, k) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<usize>(),
    );
    let mut cube = vec![vec![vec![' '; m + 2]; n + 2]; k + 2];

    for i in 1..=k {
        for j in 1..=n {
            let line = scan.line().trim().to_string();

            for (l, c) in line.chars().enumerate() {
                cube[i][j][l + 1] = c;
            }
        }
    }

    let mut visited = vec![vec![vec![false; m + 1]; n + 1]; k + 1];
    let mut set = HashSet::new();

    for i in 1..=k {
        for j in 1..=n {
            for l in 1..=m {
                if cube[i][j][l] == '.' {
                    continue;
                }

                process_dfs(&cube, &mut visited, &mut set, i, j, l, 1, 2, 3, 4);
            }
        }
    }

    writeln!(out, "{}", if set.len() == 8 { "Yes" } else { "No" }).unwrap();
}
