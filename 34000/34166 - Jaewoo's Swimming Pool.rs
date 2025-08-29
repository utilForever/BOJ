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

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[inline]
fn adj(a: &Point, b: &Point) -> bool {
    (a.x - b.x).abs() + (a.y - b.y).abs() == 1
}

fn build_even_rows(r: i32, c: i32) -> Vec<Point> {
    let mut cycle = vec![Point::new(1, 1)];
    let mut y = 1;

    while y <= r {
        for x in 2..=c {
            cycle.push(Point::new(x, y));
        }

        cycle.push(Point::new(c, y + 1));

        for x in (2..c).rev() {
            cycle.push(Point::new(x, y + 1));
        }

        y += 2;
    }

    cycle.push(Point::new(1, r));

    for yy in (2..r).rev() {
        cycle.push(Point::new(1, yy));
    }

    cycle
}

fn build_even_cols(r: i32, c: i32) -> Vec<Point> {
    let rows = build_even_rows(c, r);
    let mut cycle = Vec::with_capacity((r * c) as usize);

    for p in rows {
        cycle.push(Point::new(p.y, p.x));
    }

    cycle
}

fn build_odd(r: i32, c: i32) -> Vec<Point> {
    let cols = build_even_cols(r, c - 1);
    let mut cycle = Vec::with_capacity((r * c - 1) as usize);
    let last = c - 1;

    for i in 0..cols.len() {
        let a = cols[i];
        let b = cols[(i + 1) % cols.len()];

        cycle.push(a);

        if a.x == last && b.x == last && (a.y - b.y).abs() == 1 && (a.y.min(b.y) & 1) != 0 {
            let y = a.y.min(b.y);

            if a.y < b.y {
                cycle.push(Point::new(c, y));
                cycle.push(Point::new(c, y + 1));
            } else {
                cycle.push(Point::new(c, y + 1));
                cycle.push(Point::new(c, y));
            }
        }
    }

    cycle
}

fn shrink_cycle(cycle: &mut Vec<Point>, k: i32) {
    while cycle.len() as i32 > k {
        let n = cycle.len();

        for i in 0..n {
            let j = (i + 3) % n;

            if adj(&cycle[i], &cycle[j]) {
                let idx1 = (i + 1) % n;
                let idx2 = (i + 2) % n;

                let (first, second) = if idx1 > idx2 {
                    (idx1, idx2)
                } else {
                    (idx2, idx1)
                };

                cycle.remove(first);
                cycle.remove(second);

                break;
            }
        }
    }
}

fn encode(cycle: &[Point]) -> String {
    let mut ret = String::with_capacity(cycle.len());

    for i in 0..cycle.len() {
        let a = cycle[i];
        let b = cycle[(i + 1) % cycle.len()];
        let dx = b.x - a.x;
        let dy = b.y - a.y;

        if dx == 1 {
            ret.push('R');
        }

        if dx == -1 {
            ret.push('L');
        }

        if dy == 1 {
            ret.push('U');
        }

        if dy == -1 {
            ret.push('D');
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i32>();

    for _ in 0..t {
        let (c, r, k) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );

        if r == 1 || c == 1 || k % 2 != 0 || k < 4 {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let len_max = if (r * c) % 2 == 0 { r * c } else { r * c - 1 };

        if k > len_max {
            writeln!(out, "-1").unwrap();
            continue;
        }

        let mut cycle = if r % 2 == 0 {
            build_even_rows(r, c)
        } else if c % 2 == 0 {
            build_even_cols(r, c)
        } else {
            build_odd(r, c)
        };

        if cycle.len() as i32 > k {
            shrink_cycle(&mut cycle, k);
        }

        writeln!(out, "{}", encode(&cycle)).unwrap();
    }
}
