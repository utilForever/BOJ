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

fn find_path(rows: i32, cols: i32, sx: i32, sy: i32, tx: i32, ty: i32) -> Vec<Cell> {
    Vec::new()
}

fn path_to_moves(path: &Vec<Cell>) -> String {
    let mut ret = String::with_capacity(path.len() - 1);

    for i in 1..path.len() {
        let a = path[i - 1];
        let b = path[i];

        ret.push(if b.y == a.y + 1 && b.x == a.x {
            'U'
        } else if b.y == a.y - 1 && b.x == a.x {
            'D'
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
        let (r, c, xs, ys, xf, yf) = (
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
            scan.token::<i32>(),
        );
        let path = find_path(r, c, xs, ys, xf, yf);

        writeln!(out, "{}", path_to_moves(&path)).unwrap();
    }
}
