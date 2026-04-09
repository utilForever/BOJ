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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn select_corner_end_curr(corner_start_curr: Corner, direction: Direction) -> Corner {
    match corner_start_curr {
        Corner::TopLeft => match direction {
            Direction::Up | Direction::Right => Corner::TopRight,
            Direction::Down | Direction::Left => Corner::BottomLeft,
        },
        Corner::TopRight => match direction {
            Direction::Up | Direction::Left => Corner::TopLeft,
            Direction::Down | Direction::Right => Corner::BottomRight,
        },
        Corner::BottomLeft => match direction {
            Direction::Down | Direction::Right => Corner::BottomRight,
            Direction::Up | Direction::Left => Corner::TopLeft,
        },
        Corner::BottomRight => match direction {
            Direction::Down | Direction::Left => Corner::BottomLeft,
            Direction::Up | Direction::Right => Corner::TopRight,
        },
    }
}

fn select_corner_start_next(corner_end_prev: Corner, direction: Direction) -> Corner {
    match direction {
        Direction::Up => match corner_end_prev {
            Corner::TopLeft => Corner::BottomLeft,
            Corner::TopRight => Corner::BottomRight,
            _ => unreachable!(),
        },
        Direction::Down => match corner_end_prev {
            Corner::BottomLeft => Corner::TopLeft,
            Corner::BottomRight => Corner::TopRight,
            _ => unreachable!(),
        },
        Direction::Left => match corner_end_prev {
            Corner::TopLeft => Corner::TopRight,
            Corner::BottomLeft => Corner::BottomRight,
            _ => unreachable!(),
        },
        Direction::Right => match corner_end_prev {
            Corner::TopRight => Corner::TopLeft,
            Corner::BottomRight => Corner::BottomLeft,
            _ => unreachable!(),
        },
    }
}

fn has_edge(pattern: &Vec<Vec<char>>, a: (i64, i64), b: (i64, i64)) -> bool {
    let (y1, x1) = a;
    let (y2, x2) = b;

    if y1 == y2 && x1 + 1 == x2 {
        pattern[(2 * y1) as usize][(2 * x1 + 1) as usize] == '*'
    } else if y1 == y2 && x1 == x2 + 1 {
        pattern[(2 * y1) as usize][(2 * x2 + 1) as usize] == '*'
    } else if y1 + 1 == y2 && x1 == x2 {
        pattern[(2 * y1 + 1) as usize][(2 * x1) as usize] == '*'
    } else if y1 == y2 + 1 && x1 == x2 {
        pattern[(2 * y2 + 1) as usize][(2 * x1) as usize] == '*'
    } else {
        false
    }
}

fn map_to_point(corner_start: Corner, corner_end: Corner, path: (i64, i64), n: i64) -> (i64, i64) {
    let (y, x) = path;

    match (corner_start, corner_end) {
        (Corner::TopLeft, Corner::TopRight) => (y, x),
        (Corner::TopLeft, Corner::BottomLeft) => (x, y),
        (Corner::TopRight, Corner::TopLeft) => (y, n - 1 - x),
        (Corner::TopRight, Corner::BottomRight) => (x, n - 1 - y),
        (Corner::BottomLeft, Corner::TopLeft) => (n - 1 - x, y),
        (Corner::BottomLeft, Corner::BottomRight) => (n - 1 - y, x),
        (Corner::BottomRight, Corner::TopRight) => (n - 1 - x, n - 1 - y),
        (Corner::BottomRight, Corner::BottomLeft) => (n - 1 - y, n - 1 - x),
        _ => unreachable!(),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let height = 2 * n - 1;
    let mut pattern = vec![vec![' '; height]; height];

    for i in 0..height {
        let line = scan.line().to_string();

        for (j, c) in line.chars().take(height).enumerate() {
            pattern[i][j] = c;
        }
    }

    let mut base = Vec::with_capacity(n * n);
    let mut directions = Vec::with_capacity(n * n - 1);

    let mut curr = (0, 0);
    let mut prev = (-1, -1);

    base.push(curr);

    while base.len() < n * n {
        let (y, x) = curr;
        let candidates = [(y, x + 1), (y + 1, x), (y, x - 1), (y - 1, x)];
        let mut next = None;

        for &(y_next, x_next) in candidates.iter() {
            if y_next < 0 || y_next >= n as i64 || x_next < 0 || x_next >= n as i64 {
                continue;
            }

            if has_edge(&pattern, curr, (y_next, x_next)) && (y_next, x_next) != prev {
                next = Some((y_next, x_next));
                break;
            }
        }

        let next = next.unwrap();

        let dir = if next.0 == y && next.1 == x + 1 {
            Direction::Right
        } else if next.0 == y + 1 && next.1 == x {
            Direction::Down
        } else if next.0 == y && next.1 == x - 1 {
            Direction::Left
        } else {
            Direction::Up
        };

        prev = curr;
        curr = next;

        directions.push(dir);
        base.push(curr);
    }

    let mut corner_start = vec![Corner::TopLeft; n * n];
    let mut corner_end = vec![Corner::TopLeft; n * n];

    for i in 0..n * n {
        if i + 1 < n * n {
            corner_end[i] = select_corner_end_curr(corner_start[i], directions[i]);
            corner_start[i + 1] = select_corner_start_next(corner_end[i], directions[i]);
        } else {
            corner_end[i] = Corner::TopRight;
        }
    }

    let mut path = base.clone();
    let mut len_side = n as i64;

    for _ in 2..=m {
        let mut path_next = Vec::with_capacity(path.len() * n * n);

        for (idx, &(base_y, base_x)) in base.iter().enumerate() {
            let (offset_y, offset_x) = (base_y * len_side, base_x * len_side);

            for &(path_y, path_x) in path.iter() {
                let (y, x) = map_to_point(
                    corner_start[idx],
                    corner_end[idx],
                    (path_y, path_x),
                    len_side,
                );
                path_next.push((offset_y + y, offset_x + x));
            }
        }

        path = path_next;
        len_side *= n as i64;
    }

    let mut ret = vec![vec![' '; 2 * len_side as usize - 1]; 2 * len_side as usize - 1];

    for &(y, x) in path.iter() {
        ret[2 * y as usize][2 * x as usize] = '*';
    }

    for i in 0..path.len() - 1 {
        let (y1, x1) = path[i];
        let (y2, x2) = path[i + 1];
        let y = 2 * y1 + (y2 - y1);
        let x = 2 * x1 + (x2 - x1);

        ret[y as usize][x as usize] = '*';
    }

    for i in 0..2 * len_side as usize - 1 {
        for j in 0..2 * len_side as usize - 1 {
            write!(out, "{}", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
