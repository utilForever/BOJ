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

fn process_dfs(
    map: &Vec<Vec<i64>>,
    is_virus: &mut Vec<Vec<bool>>,
    n: usize,
    m: usize,
    x: usize,
    y: usize,
) -> i64 {
    let mut ret = 1;
    is_virus[x][y] = true;

    let dx: [i64; 4] = [1, 0, -1, 0];
    let dy: [i64; 4] = [0, 1, 0, -1];

    for i in 0..4 {
        let next_x = x as i64 + dx[i];
        let next_y = y as i64 + dy[i];

        if next_x < 0 || next_x >= n as i64 || next_y < 0 || next_y >= m as i64 {
            continue;
        }

        if is_virus[next_x as usize][next_y as usize] || map[next_x as usize][next_y as usize] != 0
        {
            continue;
        }

        ret += process_dfs(map, is_virus, n, m, next_x as usize, next_y as usize);
    }

    ret
}

fn setup_wall(
    map: &mut Vec<Vec<i64>>,
    viruses: &Vec<(usize, usize)>,
    n: usize,
    m: usize,
    ret: &mut i64,
    num_wall: i64,
    x: usize,
    y: usize,
) {
    if num_wall == 3 {
        let mut is_virus = vec![vec![false; m]; n];
        let mut cnt = 0;

        for virus in viruses {
            cnt += process_dfs(&map, &mut is_virus, n, m, virus.0, virus.1);
        }

        if *ret > cnt {
            *ret = cnt;
        }

        return;
    }

    for i in x..n {
        let y = if i == x { y } else { 0 };
        
        for j in y..m {
            if map[i][j] == 0 {
                map[i][j] = 1;
                setup_wall(map, viruses, n, m, ret, num_wall + 1, i, j + 1);
                map[i][j] = 0;
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut map = vec![vec![0; m]; n];
    let mut viruses = Vec::new();
    let mut num_rooms = 0;
    let mut ret = i64::MAX;

    for i in 0..n {
        for j in 0..m {
            map[i][j] = scan.token::<i64>();

            if map[i][j] != 1 {
                num_rooms += 1;
            }

            if map[i][j] == 2 {
                viruses.push((i, j));
            }
        }
    }

    setup_wall(&mut map, &viruses, n, m, &mut ret, 0, 0, 0);

    writeln!(out, "{}", num_rooms - 3 - ret).unwrap();
}
