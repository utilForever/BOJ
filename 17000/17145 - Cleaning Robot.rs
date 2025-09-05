use io::Write;
use std::{
    collections::{HashMap, VecDeque},
    io, str,
};

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

#[derive(Clone, Copy, Debug)]
struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[inline]
fn check_internal_three_row_edge(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let parity1 = (x1 + y1) & 1;
    let parity2 = (x2 + y2) & 1;

    if parity1 == 1 && parity2 == 0 {
        if y1 == 2 && x1 < x2 {
            return true;
        }

        if y1 != 2 && x1 < x2 - 1 {
            return true;
        }
    }

    false
}

#[inline]
fn is_three_row_even_width_edge_case(
    rows: i32,
    cols: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> bool {
    if rows != 3 {
        return false;
    }

    if (cols & 1) != 0 {
        return false;
    }

    check_internal_three_row_edge(x1, y1, x2, y2)
        || check_internal_three_row_edge(x2, y2, x1, y1)
        || check_internal_three_row_edge(cols - x1 + 1, y1, cols - x2 + 1, y2)
        || check_internal_three_row_edge(cols - x2 + 1, y2, cols - x1 + 1, y1)
}

fn upper_bound(rows: i32, cols: i32, sx: i32, sy: i32, tx: i32, ty: i32) -> i32 {
    if rows > cols {
        return upper_bound(cols, rows, sy, sx, ty, tx);
    }

    if rows == 1 {
        return (sx - tx).abs() + 1;
    }

    if rows == 2 && (sx - tx).abs() <= 1 && sy != ty {
        return (sx + tx).max(2 * cols - sx - tx + 2);
    }

    let parity1 = (sx + sy) & 1;
    let parity2 = (tx + ty) & 1;
    let special = is_three_row_even_width_edge_case(rows, cols, sx, sy, tx, ty);

    let area = rows * cols;

    if (area & 1) == 0 && parity1 != parity2 && !special {
        return area;
    }

    if (area & 1) == 1 && parity1 == 0 && parity2 == 0 && !special {
        return area;
    }

    if (area & 1) == 0 && parity1 == parity2 {
        return area - 1;
    }

    if (area & 1) == 1 && parity1 != parity2 {
        return area - 1;
    }

    if (area & 1) == 1 && parity1 == 1 && parity2 == 1 {
        return area - 2;
    }

    if (area & 1) == 0 && parity1 != parity2 && special {
        return area - 2;
    }

    if (area & 1) == 1 && parity1 == 0 && parity2 == 0 && special {
        return area - 2;
    }

    unreachable!("Unexpected case");
}

#[inline]
fn to_index(x0: i32, y0: i32, _rows: i32, cols: i32) -> usize {
    (y0 as usize) * (cols as usize) + (x0 as usize)
}

#[inline]
fn from_index(idx: usize, _rows: i32, cols: i32) -> (i32, i32) {
    let x = (idx % (cols as usize)) as i32;
    let y = (idx / (cols as usize)) as i32;
    (x, y)
}

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];

fn find_path_bruteforce(rows: i32, cols: i32, sx: i32, sy: i32, tx: i32, ty: i32) -> Vec<Cell> {
    let sx0 = sx - 1;
    let sy0 = sy - 1;
    let tx0 = tx - 1;
    let ty0 = ty - 1;

    let start = to_index(sx0, sy0, rows, cols);
    let end = to_index(tx0, ty0, rows, cols);

    let mut queue = VecDeque::new();
    let mut prev = HashMap::new();
    let mask_start = 1u64 << start;

    prev.insert((mask_start, start), usize::MAX);
    queue.push_back((mask_start, start));

    let mut mask_best = 0u64;

    while let Some((mask, pos)) = queue.pop_front() {
        if pos == end {
            if mask.count_ones() > mask_best.count_ones() {
                mask_best = mask;
            }

            continue;
        }

        let (x, y) = from_index(pos, rows, cols);

        for dir in 0..4 {
            let x_next = x + DX[dir];
            let y_next = y + DY[dir];

            if x_next < 0 || x_next >= cols || y_next < 0 || y_next >= rows {
                continue;
            }

            let pos_next = to_index(x_next, y_next, rows, cols);
            let bit = 1u64 << pos_next;

            if (mask & bit) != 0 {
                continue;
            }

            let mask_next = mask | bit;

            if prev.contains_key(&(mask_next, pos_next)) {
                continue;
            }

            prev.insert((mask_next, pos_next), pos);
            queue.push_back((mask_next, pos_next));
        }
    }

    let mut path = Vec::new();

    if mask_best == 0 {
        return path;
    }

    let mut mask = mask_best;
    let mut pos = end;

    while mask != 0 {
        let (x0, y0) = from_index(pos, rows, cols);
        path.push(Cell::new(x0 + 1, y0 + 1));

        let pos_prev = *prev.get(&(mask, pos)).unwrap();

        if pos_prev == usize::MAX {
            break;
        }

        mask ^= 1u64 << pos;
        pos = pos_prev;
    }

    path.reverse();
    path
}

fn find_path(rows: i32, cols: i32, sx: i32, sy: i32, tx: i32, ty: i32) -> Vec<Cell> {
    find_path_impl(rows, cols, sx, sy, tx, ty, false)
}

fn find_path_impl(
    rows: i32,
    cols: i32,
    sx: i32,
    sy: i32,
    tx: i32,
    ty: i32,
    flipped: bool,
) -> Vec<Cell> {
    let upper = upper_bound(rows, cols, sx, sy, tx, ty);

    for col in 1..cols {
        if col >= sx && col < tx {
            for row in 1..=rows {
                if (sx == col && sy == row) || (tx == col + 1 && ty == row) {
                    continue;
                }

                let upper1 = upper_bound(rows, col, sx, sy, col, row);
                let upper2 = upper_bound(rows, cols - col, 1, row, tx - col, ty);

                if upper1 + upper2 == upper {
                    let mut left = find_path_impl(rows, col, sx, sy, col, row, false);
                    let right = find_path_impl(rows, cols - col, 1, row, tx - col, ty, false);

                    for cell in right {
                        left.push(Cell::new(cell.x + col, cell.y));
                    }

                    return left;
                }
            }
        }
    }

    for row in 1..rows {
        if row >= sy && row < ty {
            for col in 1..=cols {
                if (sx == col && sy == row) || (tx == col && ty == row + 1) {
                    continue;
                }

                let upper1 = upper_bound(row, cols, sx, sy, col, row);
                let upper2 = upper_bound(rows - row, cols, col, 1, tx, ty - row);

                if upper1 + upper2 == upper {
                    let mut bottom = find_path_impl(row, cols, sx, sy, col, row, false);
                    let top = find_path_impl(rows - row, cols, col, 1, tx, ty - row, false);

                    for cell in top {
                        bottom.push(Cell::new(cell.x, cell.y + row));
                    }

                    return bottom;
                }
            }
        }
    }

    if flipped {
        for i in 2..sx.min(tx) {
            if (i * rows) & 1 != 0 {
                continue;
            }

            let upper2 = upper_bound(rows, cols - i, sx - i, sy, tx - i, ty);

            if upper2 + i * rows == upper {
                let base = find_path_impl(rows, cols - i, sx - i, sy, tx - i, ty, false);
                let mut merged = Vec::new();
                let mut fixed = false;

                for j in 0..base.len() {
                    if !fixed && j > 0 && base[j].x == 1 && base[j - 1].x == 1 {
                        let bridge = find_path_impl(rows, i, i, base[j - 1].y, i, base[j].y, false);

                        for b in bridge {
                            merged.push(b);
                        }

                        fixed = true;
                    }

                    merged.push(Cell::new(base[j].x + i, base[j].y));
                }

                if fixed {
                    return merged;
                }
            }
        }

        for i in (sx.max(tx) + 1)..=(cols - 2) {
            if ((cols - i) * rows) & 1 != 0 {
                continue;
            }

            let upper2 = upper_bound(rows, i, sx, sy, tx, ty);

            if upper2 + (cols - i) * rows == upper {
                let base = find_path_impl(rows, i, sx, sy, tx, ty, false);
                let mut merged = Vec::new();
                let mut fixed = false;

                for j in 0..base.len() {
                    if !fixed && j > 0 && base[j].x == i && base[j - 1].x == i {
                        let bridge =
                            find_path_impl(rows, cols - i, 1, base[j - 1].y, 1, base[j].y, false);

                        for b in bridge {
                            merged.push(Cell { x: b.x + i, y: b.y });
                        }

                        fixed = true;
                    }

                    merged.push(base[j]);
                }

                if fixed {
                    return merged;
                }
            }
        }

        for i in 2..sy.min(ty) {
            if (i * cols) & 1 != 0 {
                continue;
            }

            let upper2 = upper_bound(rows - i, cols, sx, sy - i, tx, ty - i);

            if upper2 + i * cols == upper {
                let base = find_path_impl(rows - i, cols, sx, sy - i, tx, ty - i, false);
                let mut merged = Vec::new();
                let mut fixed = false;

                for j in 0..base.len() {
                    if !fixed && j > 0 && base[j].y == 1 && base[j - 1].y == 1 {
                        let bridge = find_path_impl(i, cols, base[j - 1].x, i, base[j].x, i, false);

                        for b in bridge {
                            merged.push(b);
                        }

                        fixed = true;
                    }

                    merged.push(Cell::new(base[j].x, base[j].y + i));
                }

                if fixed {
                    return merged;
                }
            }
        }

        for i in (sy.max(ty) + 1)..=(rows - 2) {
            if ((rows - i) * cols) & 1 != 0 {
                continue;
            }

            let upper2 = upper_bound(i, cols, sx, sy, tx, ty);

            if upper2 + (rows - i) * cols == upper {
                let base = find_path_impl(i, cols, sx, sy, tx, ty, false);
                let mut merged = Vec::new();
                let mut fixed = false;

                for j in 0..base.len() {
                    if !fixed && j > 0 && base[j].y == i && base[j - 1].y == i {
                        let bridge =
                            find_path_impl(rows - i, cols, base[j - 1].x, 1, base[j].x, 1, false);

                        for b in bridge {
                            merged.push(Cell { x: b.x, y: b.y + i });
                        }

                        fixed = true;
                    }

                    merged.push(base[j]);
                }

                if fixed {
                    return merged;
                }
            }
        }
    }

    if !flipped {
        let mut rev = find_path_impl(rows, cols, tx, ty, sx, sy, true);
        rev.reverse();
        rev
    } else {
        find_path_bruteforce(rows, cols, sx, sy, tx, ty)
    }
}

fn path_to_moves(path: &Vec<Cell>) -> String {
    let mut ret = String::with_capacity(path.len() - 1);

    for i in 1..path.len() {
        let a = path[i - 1];
        let b = path[i];

        ret.push(if b.y == a.y + 1 && b.x == a.x {
            'D'
        } else if b.y == a.y - 1 && b.x == a.x {
            'U'
        } else if b.x == a.x + 1 && b.y == a.y {
            'R'
        } else if b.x == a.x - 1 && b.y == a.y {
            'L'
        } else {
            unreachable!("Invalid path")
        });
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (r, c, ys, xs) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );

        let mut check = false;

        'outer: for i in 1..=r {
            for j in 1..=c {
                if i == ys && j == xs {
                    continue;
                }

                let dist = upper_bound(r, c, xs, ys, j, i);

                if dist == r * c {
                    let path = find_path(r, c, xs, ys, j, i);

                    writeln!(out, "{}", path_to_moves(&path)).unwrap();

                    check = true;
                    break 'outer;
                }
            }
        }

        if !check {
            writeln!(out, "IMPOSSIBLE").unwrap();
        }
    }
}
