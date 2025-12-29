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

fn build_teacher(n: usize) -> Vec<(usize, usize)> {
    let mut positions = Vec::with_capacity(2 * n);
    let mut r = n;
    let mut c = 1;

    r += 1;
    positions.push((r, c));
    r -= 1;
    positions.push((r, c));

    for col in 2..=n {
        if col % 2 == 0 {
            c += 1;
            positions.push((r, c));
            r += 1;
            positions.push((r, c));
        } else {
            c += 1;
            positions.push((r, c));
            r -= 1;
            positions.push((r, c));
        }
    }

    positions
}

fn build_path_top(n: usize, col: usize) -> Vec<(usize, usize)> {
    let len = n - col;
    let mut path = Vec::with_capacity(2 * len + 1);

    path.push((n, col));

    for i in 1..=len {
        path.push((n - i, col));
    }

    let row = col;

    for i in 1..=len {
        path.push((row, col + i));
    }

    path
}

fn build_path_bottom(n: usize, col: usize) -> Vec<(usize, usize)> {
    let len = n - col;
    let mut path = Vec::with_capacity(2 * len + 1);

    path.push((n + 1, col));

    for i in 1..=len {
        path.push((n + 1 + i, col));
    }

    let row = 2 * n - col + 1;

    for i in 1..=len {
        path.push((row, col + i));
    }

    path
}

fn direction(from: (usize, usize), to: (usize, usize)) -> char {
    let (from_r, from_c) = from;
    let (to_r, to_c) = to;
    let (diff_r, diff_c) = (
        to_r as isize - from_r as isize,
        to_c as isize - from_c as isize,
    );

    match (diff_r, diff_c) {
        (0, 1) => 'R',
        (0, -1) => 'L',
        (1, 0) => 'D',
        (-1, 0) => 'U',
        _ => unreachable!(),
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let positions_teacher = build_teacher(n);

    let mut time_top = vec![0; n + 1];
    let mut time_bot = vec![0; n + 1];

    for (idx, &(r, c)) in positions_teacher.iter().enumerate() {
        let time = idx + 1;

        if r == n {
            time_top[c] = time;
        } else {
            time_bot[c] = time;
        }
    }

    let mut positions_student = vec![Vec::new(); 2 * n + 1];

    for col in 1..=n {
        {
            let time_start = time_top[col];
            let path = build_path_top(n, col);

            for i in 0..path.len() - 1 {
                let from = path[i];
                let to = path[i + 1];
                let time = time_start + 1 + i;

                if time <= 2 * n {
                    positions_student[time as usize].push((from.0, from.1, direction(from, to)));
                }
            }
        }

        {
            let time_start = time_bot[col];
            let path = build_path_bottom(n, col);

            for i in 0..path.len() - 1 {
                let from = path[i];
                let to = path[i + 1];
                let time = time_start + 1 + i;

                if time <= 2 * n {
                    positions_student[time as usize].push((from.0, from.1, direction(from, to)));
                }
            }
        }
    }

    writeln!(out, "{n} 1").unwrap();
    writeln!(out, "{}", 2 * n).unwrap();

    for i in 1..=2 * n {
        writeln!(out, "{}", positions_student[i].len()).unwrap();

        for &(r, c, d) in positions_student[i].iter() {
            writeln!(out, "{r} {c} {d}").unwrap();
        }

        let prev = if i == 1 {
            (n, 1)
        } else {
            positions_teacher[i - 2]
        };
        let curr = positions_teacher[i - 1];

        writeln!(out, "{} {}", direction(prev, curr), 1_000_000_000).unwrap();
    }
}
