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

fn process_greedy(pots: &Vec<usize>, pots_inv: &Vec<usize>) -> Option<Vec<char>> {
    let n = pots.len();
    let mut ret = vec![' '; n];

    let mark = |ret: &mut Vec<char>, row: i32, c: char| {
        if row < 0 || row >= n as i32 {
            return;
        }

        let col = pots_inv[row as usize];
        ret[col] = c;
    };

    let mut left = -1;
    let mut mid = pots[0] as i32;
    let mut right = n as i32;

    // From left to right
    for &r in pots[1..].iter() {
        let r = r as i32;

        if r < left || r > right {
            break;
        }

        if r < mid {
            mark(&mut ret, right, 'C');
            right = mid;
            mid = r;
        } else {
            mark(&mut ret, left, 'E');
            left = mid;
            mid = r;
        }
    }

    mark(&mut ret, left, 'E');
    mark(&mut ret, mid, 'E');
    mark(&mut ret, right, 'C');

    left = -1;
    mid = pots[n - 1] as i32;
    right = n as i32;

    // From right to left
    for &r in pots[..n - 1].iter().rev() {
        let r = r as i32;

        if r < left || r > right {
            break;
        }

        if r < mid {
            mark(&mut ret, right, 'Z');
            right = mid;
            mid = r;
        } else {
            mark(&mut ret, left, 'Q');
            left = mid;
            mid = r;
        }
    }

    mark(&mut ret, left, 'Q');
    mark(&mut ret, mid, 'Q');
    mark(&mut ret, right, 'Z');

    if ret.iter().all(|&c| c != ' ') {
        Some(ret)
    } else {
        None
    }
}

fn is_succ(u: (i32, i32), v: (i32, i32), dir: usize) -> bool {
    match dir {
        0 | 1 => u.0 <= v.0 && u.1 >= v.1, // x+, y-
        2 | 3 => u.0 >= v.0 && u.1 >= v.1, // x-, y-
        4 | 5 => u.0 >= v.0 && u.1 <= v.1, // x-, y+
        6 | 7 => u.0 <= v.0 && u.1 <= v.1, // x+, y+
        _ => unreachable!(),
    }
}

fn build_chains(pots: &Vec<usize>, pots_inv: &Vec<usize>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let n = pots.len();
    let mut chain_len = vec![vec![0; n]; 8];
    let mut point_prev = vec![vec![-1; n]; 8];

    for i in 0..8 {
        let mut stack = Vec::new();

        if i == 0 || i == 7 {
            // x+
            for j in 0..n {
                let point_curr = (j as i32, pots[j] as i32);

                while let Some(&last) = stack.last() {
                    let point_last = (last as i32, pots[last] as i32);

                    if !is_succ(point_last, point_curr, i) {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                chain_len[i][j] = stack.len() as i32 + 1;
                point_prev[i][j] = stack.last().map(|&x| x as i32).unwrap_or(-1);
                stack.push(j);
            }
        } else if i == 3 || i == 4 {
            // x-
            for j in (0..n).rev() {
                let point_curr = (j as i32, pots[j] as i32);

                while let Some(&last) = stack.last() {
                    let point_last = (last as i32, pots[last] as i32);

                    if !is_succ(point_last, point_curr, i) {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                chain_len[i][j] = stack.len() as i32 + 1;
                point_prev[i][j] = stack.last().map(|&x| x as i32).unwrap_or(-1);
                stack.push(j);
            }
        } else if i == 1 || i == 2 {
            // y-
            for j in (0..n).rev() {
                let point_curr = (pots_inv[j] as i32, j as i32);

                while let Some(&last) = stack.last() {
                    let point_last = (last as i32, pots[last] as i32);

                    if !is_succ(point_last, point_curr, i) {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                chain_len[i][pots_inv[j]] = stack.len() as i32 + 1;
                point_prev[i][pots_inv[j]] = stack.last().map(|&x| x as i32).unwrap_or(-1);
                stack.push(pots_inv[j]);
            }
        } else {
            // y+
            for j in 0..n {
                let point_curr = (pots_inv[j] as i32, j as i32);

                while let Some(&last) = stack.last() {
                    let point_last = (last as i32, pots[last] as i32);

                    if !is_succ(point_last, point_curr, i) {
                        stack.pop();
                    } else {
                        break;
                    }
                }

                chain_len[i][pots_inv[j]] = stack.len() as i32 + 1;
                point_prev[i][pots_inv[j]] = stack.last().map(|&x| x as i32).unwrap_or(-1);
                stack.push(pots_inv[j]);
            }
        }
    }

    (chain_len, point_prev)
}

fn get_point_prev(point_prev: &Vec<Vec<i32>>, dir: usize, col: usize) -> Option<usize> {
    if point_prev[dir][col] < 0 {
        None
    } else {
        Some(point_prev[dir][col] as usize)
    }
}

fn check_pattern_odd(
    chain_len: &Vec<Vec<i32>>,
    point_prev: &Vec<Vec<i32>>,
    a: usize,
    n: usize,
) -> Option<(usize, usize, usize, usize)> {
    let b = get_point_prev(point_prev, 5, a)?;
    let c = get_point_prev(point_prev, 7, b)?;
    let mut d = get_point_prev(point_prev, 1, c)?;

    if d == a {
        d = get_point_prev(point_prev, 1, d)?;
    }

    if d < a {
        let total = chain_len[1][d] + chain_len[3][a] + chain_len[5][b] + chain_len[7][c];

        if total as usize == n {
            return Some((d, a, b, c));
        }
    }

    None
}

fn check_pattern_even(
    chain_len: &Vec<Vec<i32>>,
    point_prev: &Vec<Vec<i32>>,
    a: usize,
    n: usize,
) -> Option<(usize, usize, usize, usize)> {
    let b = get_point_prev(point_prev, 6, a)?;
    let c = get_point_prev(point_prev, 4, b)?;
    let mut d = get_point_prev(point_prev, 2, c)?;

    if d == a {
        d = get_point_prev(point_prev, 2, d)?;
    }

    if a < d {
        let total = chain_len[0][a] + chain_len[2][d] + chain_len[4][c] + chain_len[6][b];

        if total as usize == n {
            return Some((a, d, c, b));
        }
    }

    None
}

fn find_pivot(
    chain_len: &Vec<Vec<i32>>,
    point_prev: &Vec<Vec<i32>>,
) -> Option<(usize, usize, usize, usize)> {
    let n = chain_len[0].len();

    for i in 0..n {
        if let Some(pivot) = check_pattern_odd(chain_len, point_prev, i, n) {
            return Some(pivot);
        }

        if let Some(pivot) = check_pattern_even(chain_len, point_prev, i, n) {
            return Some(pivot);
        }
    }

    None
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut pots = vec![0; n];

    for i in 0..n {
        pots[i] = scan.token::<usize>() - 1;
    }

    let mut pots_inv = vec![0; n];

    for (r, &c) in pots.iter().enumerate() {
        pots_inv[c] = r;
    }

    if let Some(ret) = process_greedy(&pots, &pots_inv) {
        writeln!(out, "YES").unwrap();

        for val in ret {
            write!(out, "{val}").unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    if let Some(mut ret) = process_greedy(&pots_inv, &pots) {
        writeln!(out, "YES").unwrap();

        for val in ret.iter_mut() {
            match *val {
                'Q' => *val = 'C',
                'C' => *val = 'Q',
                _ => {}
            }
        }

        let mut remapped = vec![' '; n];

        for i in 0..n {
            remapped[pots_inv[i]] = ret[i];
        }

        for val in remapped {
            write!(out, "{val}").unwrap();
        }

        writeln!(out).unwrap();
        return;
    }

    let (len, prev) = build_chains(&pots, &pots_inv);

    if let Some(pivot) = find_pivot(&len, &prev) {
        writeln!(out, "YES").unwrap();

        for i in 0..n {
            let point_curr = (i as i32, pots[i] as i32);
            write!(
                out,
                "{}",
                if is_succ(point_curr, (pivot.0 as i32, pots[pivot.0] as i32), 0) {
                    'C'
                } else if is_succ(point_curr, (pivot.1 as i32, pots[pivot.1] as i32), 2) {
                    'Z'
                } else if is_succ(point_curr, (pivot.2 as i32, pots[pivot.2] as i32), 4) {
                    'Q'
                } else {
                    'E'
                }
            )
            .unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}
