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

fn nqueens_columns(n: usize) -> Vec<usize> {
    let mut evens = (2..=n).step_by(2).collect::<Vec<usize>>();
    let mut odds = (1..=n).step_by(2).collect::<Vec<usize>>();

    match n % 6 {
        2 => {
            if let (Some(i1), Some(i3)) = (
                odds.iter().position(|&x| x == 1),
                odds.iter().position(|&x| x == 3),
            ) {
                odds.swap(i1, i3);
            }

            if let Some(i5) = odds.iter().position(|&x| x == 5) {
                let five = odds.remove(i5);
                odds.push(five);
            }
        }
        3 => {
            if let Some(i2) = evens.iter().position(|&x| x == 2) {
                let two = evens.remove(i2);
                evens.push(two);
            }

            for t in [1, 3] {
                if let Some(pos) = odds.iter().position(|&x| x == t) {
                    let v = odds.remove(pos);
                    odds.push(v);
                }
            }
        }
        _ => {}
    }

    let seq = evens
        .into_iter()
        .chain(odds.into_iter())
        .collect::<Vec<usize>>();
    let mut ret = vec![0; n];

    for (c, r) in seq.into_iter().enumerate() {
        ret[r - 1] = c;
    }

    ret
}

fn assign_single(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    *id_next += 1;
    zone[y as usize][x as usize] = *id_next;
}

fn assign_pair(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y1: i64, x1: i64, y2: i64, x2: i64) {
    *id_next += 1;
    zone[y1 as usize][x1 as usize] = *id_next;
    zone[y2 as usize][x2 as usize] = *id_next;
}

fn pattern_2x4(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 1, x + 2);
    assign_pair(zone, id_next, y, x + 1, y + 1, x + 3);
    assign_pair(zone, id_next, y, x + 2, y + 1, x);
    assign_pair(zone, id_next, y, x + 3, y + 1, x + 1);
}

fn pattern_4x2(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 2, x + 1);
    assign_pair(zone, id_next, y, x + 1, y + 2, x);
    assign_pair(zone, id_next, y + 1, x, y + 3, x + 1);
    assign_pair(zone, id_next, y + 1, x + 1, y + 3, x);
}

fn pattern_3x4(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 1, x + 2);
    assign_pair(zone, id_next, y + 1, x, y, x + 2);
    assign_pair(zone, id_next, y + 2, x, y, x + 1);
    assign_pair(zone, id_next, y + 1, x + 1, y + 2, x + 3);
    assign_pair(zone, id_next, y + 2, x + 1, y + 1, x + 3);
    assign_pair(zone, id_next, y + 2, x + 2, y, x + 3);
}

fn pattern_4x3(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 2, x + 1);
    assign_pair(zone, id_next, y, x + 1, y + 2, x);
    assign_pair(zone, id_next, y, x + 2, y + 1, x);
    assign_pair(zone, id_next, y + 1, x + 1, y + 3, x + 2);
    assign_pair(zone, id_next, y + 1, x + 2, y + 3, x + 1);
    assign_pair(zone, id_next, y + 2, x + 2, y + 3, x);
}

fn pattern_3x3(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 1, x + 2);
    assign_pair(zone, id_next, y + 1, x, y + 2, x + 2);
    assign_pair(zone, id_next, y + 2, x, y, x + 1);
    assign_pair(zone, id_next, y + 2, x + 1, y, x + 2);
    assign_single(zone, id_next, y + 1, x + 1);
}

fn pattern_5x5(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x + 3, y + 1, x + 1);
    assign_pair(zone, id_next, y, x, y + 1, x + 2);
    assign_pair(zone, id_next, y, x + 1, y + 1, x + 3);
    assign_pair(zone, id_next, y, x + 2, y + 1, x + 4);
    assign_pair(zone, id_next, y, x + 4, y + 2, x + 3);
    assign_pair(zone, id_next, y + 1, x, y + 3, x + 1);
    assign_pair(zone, id_next, y + 2, x, y + 4, x + 1);
    assign_pair(zone, id_next, y + 2, x + 1, y + 3, x + 3);
    assign_pair(zone, id_next, y + 2, x + 2, y + 3, x + 4);
    assign_pair(zone, id_next, y + 2, x + 4, y + 4, x + 3);
    assign_pair(zone, id_next, y + 3, x, y + 4, x + 2);
    assign_pair(zone, id_next, y + 3, x + 2, y + 4, x);
    assign_single(zone, id_next, y + 4, x + 4);
}

fn pattern_6x6(zone: &mut Vec<Vec<i64>>, id_next: &mut i64, y: i64, x: i64) {
    assign_pair(zone, id_next, y, x, y + 2, x + 1);
    assign_pair(zone, id_next, y, x + 1, y + 1, x + 3);
    assign_pair(zone, id_next, y, x + 2, y + 1, x + 4);
    assign_pair(zone, id_next, y, x + 3, y + 1, x + 5);
    assign_pair(zone, id_next, y, x + 4, y + 1, x + 2);
    assign_pair(zone, id_next, y, x + 5, y + 2, x + 4);

    assign_pair(zone, id_next, y + 1, x, y + 2, x + 2);
    assign_pair(zone, id_next, y + 1, x + 1, y + 3, x);

    assign_pair(zone, id_next, y + 2, x, y + 3, x + 2);
    assign_pair(zone, id_next, y + 2, x + 3, y + 3, x + 5);
    assign_pair(zone, id_next, y + 2, x + 5, y + 4, x + 4);

    assign_pair(zone, id_next, y + 3, x + 1, y + 5, x);
    assign_pair(zone, id_next, y + 3, x + 3, y + 4, x + 1);
    assign_pair(zone, id_next, y + 3, x + 4, y + 5, x + 5);

    assign_pair(zone, id_next, y + 4, x, y + 5, x + 2);
    assign_pair(zone, id_next, y + 4, x + 2, y + 5, x + 4);
    assign_pair(zone, id_next, y + 4, x + 3, y + 5, x + 1);
    assign_pair(zone, id_next, y + 4, x + 5, y + 5, x + 3);
}

fn solve_knight_zones(
    zone: &mut Vec<Vec<i64>>,
    id_next: &mut i64,
    y1: i64,
    x1: i64,
    y2: i64,
    x2: i64,
) {
    let h = y2 - y1 + 1;
    let w = x2 - x1 + 1;

    match (h, w) {
        (1, 1) => {
            assign_single(zone, id_next, y1, x1);
            return;
        }
        (2, 4) => {
            pattern_2x4(zone, id_next, y1, x1);
            return;
        }
        (4, 2) => {
            pattern_4x2(zone, id_next, y1, x1);
            return;
        }
        (3, 4) => {
            pattern_3x4(zone, id_next, y1, x1);
            return;
        }
        (4, 3) => {
            pattern_4x3(zone, id_next, y1, x1);
            return;
        }
        (3, 3) => {
            pattern_3x3(zone, id_next, y1, x1);
            return;
        }
        (4, 4) => {
            pattern_2x4(zone, id_next, y1, x1);
            pattern_2x4(zone, id_next, y1 + 2, x1);
            return;
        }
        (5, 5) => {
            pattern_5x5(zone, id_next, y1, x1);
            return;
        }
        (6, 6) => {
            pattern_6x6(zone, id_next, y1, x1);
            return;
        }
        _ => {}
    }

    if h >= 7 && w >= 7 {
        solve_knight_zones(zone, id_next, y1, x1, y1 + 3, x1 + 3);
        solve_knight_zones(zone, id_next, y1 + 4, x1 + 4, y2, x2);

        let mut d = 4;

        while d < h {
            if d + 3 == h {
                solve_knight_zones(zone, id_next, y1, x2 - 2, y1 + 3, x2);
                solve_knight_zones(zone, id_next, y2 - 2, x1, y2, x1 + 3);
                return;
            } else {
                solve_knight_zones(zone, id_next, y1, x1 + d, y1 + 3, x1 + d + 1);
                solve_knight_zones(zone, id_next, y1 + d, x1, y1 + d + 1, x1 + 3);

                d += 2;
            }
        }
        return;
    }

    if h < w {
        let w_cut = if w == 5 {
            5
        } else if w == 6 {
            6
        } else if w % 4 == 0 {
            4
        } else {
            3
        };
        let xr = x2 - (w_cut - 1);

        solve_knight_zones(zone, id_next, y1, xr, y2, x2);
        solve_knight_zones(zone, id_next, y1, x1, y2, xr - 1);
    } else {
        let h_cut = if h == 5 {
            5
        } else if h == 6 {
            6
        } else if h % 4 == 0 {
            4
        } else {
            3
        };
        let yb = y2 - (h_cut - 1);
        solve_knight_zones(zone, id_next, yb, x1, y2, x2);
        solve_knight_zones(zone, id_next, y1, x1, yb - 1, x2);
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let p = scan.token::<String>();
    let piece = match p.as_str() {
        "King" => 'K',
        "Queen" => 'Q',
        "Rook" => 'R',
        "Bishop" => 'B',
        "Knight" => 'N',
        "Pawn" => 'P',
        _ => unreachable!(),
    };

    let mut board = vec![vec!['.'; n]; n];
    let mut zone = vec![vec![0; n]; n];
    let m;

    match p.as_str() {
        "King" => {
            let c = ((n + 1) / 2) as i64;
            m = c * c;

            for i in (0..n).step_by(2) {
                for j in (0..n).step_by(2) {
                    board[i][j] = piece;
                }
            }

            let w = (n + 1) / 2;

            for i in 0..n {
                for j in 0..n {
                    let id = (i / 2) * w + (j / 2) + 1;
                    zone[i][j] = id as i64;
                }
            }
        }
        "Rook" => {
            m = n as i64;

            for i in 0..n {
                board[i][i] = piece;
            }

            for i in 0..n {
                let id = (i + 1) as i64;

                for j in 0..n {
                    zone[i][j] = id;
                }
            }
        }

        "Queen" => {
            if n == 1 {
                m = 1;
                board[0][0] = piece;
                zone[0][0] = 1;
            } else if n == 2 {
                m = 1;
                board[0][0] = piece;

                for i in 0..n {
                    for j in 0..n {
                        zone[i][j] = 1;
                    }
                }
            } else if n == 3 {
                m = 2;
                board[0][1] = piece;
                board[2][0] = piece;

                for i in 0..n {
                    for j in 0..n {
                        zone[i][j] = ((i + j) % 2 + 1) as i64;
                    }
                }
            } else {
                let cols = nqueens_columns(n);
                m = n as i64;

                for (i, &c) in cols.iter().enumerate() {
                    board[i][c] = piece;
                }

                for i in 0..n {
                    let id = (i + 1) as i64;

                    for j in 0..n {
                        zone[i][j] = id;
                    }
                }
            }
        }
        "Bishop" => {
            if n == 1 {
                m = 1;
                board[0][0] = piece;
                zone[0][0] = 1;
            } else {
                m = (2 * n as i64) - 2;

                for i in 0..(n - 1) {
                    board[i][0] = piece;
                    board[i][n - 1] = piece;
                }

                let s_max = 2 * (n - 1);

                for i in 0..n {
                    for j in 0..n {
                        let s = i + j;

                        zone[i][j] = if s == 0 || s == s_max {
                            1
                        } else {
                            (s + 1) as i64
                        };
                    }
                }
            }
        }
        "Knight" => {
            if n == 1 {
                m = 1;
                board[0][0] = piece;
                zone[0][0] = 1;
            } else if n == 2 {
                m = 4;
                let mut id = 0;

                for i in 0..n {
                    for j in 0..n {
                        board[i][j] = piece;
                        id += 1;
                        zone[i][j] = id;
                    }
                }
            } else {
                m = ((n * n + 1) / 2) as i64;

                for i in 0..n {
                    for j in 0..n {
                        if ((i + j) & 1) == 0 {
                            board[i][j] = piece;
                        }
                    }
                }

                let mut id_next = 0;
                solve_knight_zones(&mut zone, &mut id_next, 0, 0, n as i64 - 1, n as i64 - 1);
            }
        }
        "Pawn" => {
            let even_rows = (n + 1) / 2;
            m = (even_rows * n) as i64;

            for i in (0..n).step_by(2) {
                for j in 0..n {
                    board[i][j] = piece;
                }
            }

            let mut id = 0;

            if n % 2 == 0 {
                for i in (0..n).step_by(2) {
                    for j in 0..n {
                        id += 1;
                        zone[i][j] = id;

                        let jj = if (j & 1) == 0 { j + 1 } else { j - 1 };
                        zone[i + 1][jj] = id;
                    }
                }
            } else {
                for i in 0..n {
                    for j in 0..n {
                        if zone[i][j] != 0 {
                            continue;
                        }

                        if (i == 0 || j == 0) && ((i + j) % 2 == 0) {
                            id += 1;
                            zone[i][j] = id;
                        } else if i + 1 < n && j + 1 < n && (i.max(j) % 2 == 1) {
                            id += 1;
                            zone[i][j] = id;
                            zone[i + 1][j + 1] = id;
                        } else {
                            // Do nothing
                        }
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    writeln!(out, "{m}").unwrap();

    for i in 0..n {
        for j in 0..n {
            write!(out, "{}", board[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }

    for i in 0..n {
        for j in 0..n {
            write!(out, "{} ", zone[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
