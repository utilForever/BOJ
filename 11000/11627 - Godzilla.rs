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

fn process_godzilla(
    map: &mut Vec<Vec<char>>,
    visited_godzilla: &mut Vec<Vec<bool>>,
    pos_godzilla: &mut (usize, usize),
    ret: &mut i64,
) -> bool {
    // Order: North, East, South, West
    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];
    let (l, w) = (map.len(), map[0].len());
    let (y_curr, x_curr) = *pos_godzilla;

    // First, check resident
    for i in 0..4 {
        let (y_next, x_next) = (y_curr as i32 + dy[i], x_curr as i32 + dx[i]);

        if y_next < 0 || y_next >= l as i32 || x_next < 0 || x_next >= w as i32 {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        if map[y_next][x_next] != 'R' {
            continue;
        }

        *pos_godzilla = (y_next, x_next);
        map[y_next][x_next] = '.';
        visited_godzilla[y_next][x_next] = true;

        *ret += 1;

        return true;
    }

    // Second, check empty
    for i in 0..4 {
        let (y_next, x_next) = (y_curr as i32 + dy[i], x_curr as i32 + dx[i]);

        if y_next < 0 || y_next >= l as i32 || x_next < 0 || x_next >= w as i32 {
            continue;
        }

        let (y_next, x_next) = (y_next as usize, x_next as usize);

        if visited_godzilla[y_next][x_next] {
            continue;
        }

        if map[y_next][x_next] != '.' {
            continue;
        }

        *pos_godzilla = (y_next, x_next);
        visited_godzilla[y_next][x_next] = true;

        return true;
    }

    false
}

fn process_mechs(
    map: &mut Vec<Vec<char>>,
    visited_mechs: &mut Vec<Vec<bool>>,
    pos_mechs: &mut Vec<(usize, usize)>,
) {
    // Order: North, East, South, West
    let dy = [-1, 0, 1, 0];
    let dx = [0, 1, 0, -1];
    let (l, w) = (map.len(), map[0].len());
    let len_mechs = pos_mechs.len();
    let mut pos_mechs_new = Vec::new();

    for i in 0..len_mechs {
        let (y_curr, x_curr) = pos_mechs[i];

        for j in 0..4 {
            let (y_next, x_next) = (y_curr as i32 + dy[j], x_curr as i32 + dx[j]);

            if y_next < 0 || y_next >= l as i32 || x_next < 0 || x_next >= w as i32 {
                continue;
            }

            let (y_next, x_next) = (y_next as usize, x_next as usize);

            if visited_mechs[y_next][x_next] {
                continue;
            }

            if map[y_next][x_next] != '.' {
                continue;
            }

            pos_mechs_new.push((y_next, x_next));
            map[y_next][x_next] = 'M';
            visited_mechs[y_next][x_next] = true;
        }
    }

    *pos_mechs = pos_mechs_new;
}

fn can_shoot(map: &Vec<Vec<char>>, pos_godzilla: &(usize, usize)) -> bool {
    let (y_godzilla, x_godzilla) = (pos_godzilla.0 as i32, pos_godzilla.1 as i32);
    let mut offset = 0;

    // North
    while y_godzilla - offset >= 0
        && map[(y_godzilla - offset) as usize][x_godzilla as usize] == '.'
    {
        offset += 1;
    }

    if y_godzilla - offset >= 0 && map[(y_godzilla - offset) as usize][x_godzilla as usize] == 'M' {
        return true;
    }

    offset = 0;

    // East
    while x_godzilla + offset < map[0].len() as i32
        && map[y_godzilla as usize][(x_godzilla + offset) as usize] == '.'
    {
        offset += 1;
    }

    if x_godzilla + offset < map[0].len() as i32
        && map[y_godzilla as usize][(x_godzilla + offset) as usize] == 'M'
    {
        return true;
    }

    offset = 0;

    // South
    while y_godzilla + offset < map.len() as i32
        && map[(y_godzilla + offset) as usize][x_godzilla as usize] == '.'
    {
        offset += 1;
    }

    if y_godzilla + offset < map.len() as i32
        && map[(y_godzilla + offset) as usize][x_godzilla as usize] == 'M'
    {
        return true;
    }

    offset = 0;

    // West
    while x_godzilla - offset >= 0
        && map[y_godzilla as usize][(x_godzilla - offset) as usize] == '.'
    {
        offset += 1;
    }

    if x_godzilla - offset >= 0 && map[y_godzilla as usize][(x_godzilla - offset) as usize] == 'M' {
        return true;
    }

    false
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (l, w) = (scan.token::<usize>(), scan.token::<usize>());
        let mut map = vec![vec![' '; l]; w];
        let mut pos_godzilla = (0, 0);
        let mut pos_mechs = Vec::new();
        let mut visited_godzilla = vec![vec![false; l]; w];
        let mut visited_mechs = vec![vec![false; l]; w];

        for i in 0..w {
            let line = scan.token::<String>();

            for (j, c) in line.chars().enumerate() {
                map[i][j] = c;

                if c == 'G' {
                    pos_godzilla = (i, j);
                    visited_godzilla[i][j] = true;
                    map[i][j] = '.';
                } else if c == 'M' {
                    pos_mechs.push((i, j));
                    visited_mechs[i][j] = true;
                }
            }
        }

        let mut ret = 0;

        loop {
            let can_move =
                process_godzilla(&mut map, &mut visited_godzilla, &mut pos_godzilla, &mut ret);

            if !can_move {
                break;
            }

            process_mechs(&mut map, &mut visited_mechs, &mut pos_mechs);

            if can_shoot(&map, &pos_godzilla) {
                break;
            }
        }

        writeln!(out, "{ret}").unwrap();
    }
}
