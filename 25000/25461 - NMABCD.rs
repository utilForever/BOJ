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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (m, n, xs, ys, xf, yf) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );
        let dist_max = upper_bound(n, m, xs, ys, xf, yf) as usize;

        writeln!(out, "{dist_max}").unwrap();
    }
}
