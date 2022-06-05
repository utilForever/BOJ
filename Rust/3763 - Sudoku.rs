use io::Write;
use std::{
    cmp::Ordering,
    io,
    ops::{self, Range},
    str,
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
}

pub fn solve(mut m: Matrix) -> Vec<Vec<usize>> {
    let mut answers = Vec::new();
    let mut answer = Vec::new();

    solve_internal(&mut m, &mut answer, &mut answers);

    answers
}

fn solve_internal(m: &mut Matrix, partial_answer: &mut Vec<Cell>, answers: &mut Vec<Vec<usize>>) {
    let c = {
        let mut i = m.x.cursor(Cell(0));
        let mut c = match i.next(&m.x) {
            Some(it) => it,
            None => {
                let mut answer: Vec<usize> =
                    partial_answer.iter().map(|&cell| m.row_of(cell)).collect();
                answer.sort();
                answers.push(answer);
                return;
            }
        };

        while let Some(next_c) = i.next(&m.x) {
            if m.size[next_c] < m.size[c] {
                c = next_c;
            }
        }

        c
    };

    m.cover(c);

    let mut r = m.y.cursor(c);

    while let Some(r) = r.next(&m.y) {
        partial_answer.push(r);

        let mut j = m.x.cursor(r);
        while let Some(j) = j.next(&m.x) {
            m.cover(m.c[j]);
        }

        solve_internal(m, partial_answer, answers);

        let mut j = m.x.cursor(r);
        while let Some(j) = j.prev(&m.x) {
            m.uncover(m.c[j]);
        }

        partial_answer.pop();
    }

    m.uncover(c);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Cell(usize);

#[derive(Debug)]
struct Link {
    prev: Cell,
    next: Cell,
}

#[derive(Default, Debug)]
struct LinkedList {
    data: Vec<Link>,
}

impl ops::Index<Cell> for LinkedList {
    type Output = Link;

    fn index(&self, index: Cell) -> &Link {
        &self.data[index.0]
    }
}

impl ops::IndexMut<Cell> for LinkedList {
    fn index_mut(&mut self, index: Cell) -> &mut Link {
        &mut self.data[index.0]
    }
}

impl LinkedList {
    fn with_capacity(cap: usize) -> Self {
        Self {
            data: Vec::with_capacity(cap),
        }
    }

    fn alloc(&mut self) -> Cell {
        let cell = Cell(self.data.len());
        self.data.push(Link {
            prev: cell,
            next: cell,
        });
        cell
    }

    /// Inserts `b` into `a <-> c` to get `a <-> b <-> c`
    fn insert(&mut self, a: Cell, b: Cell) {
        let c = self[a].next;

        self[b].prev = a;
        self[b].next = c;

        self[a].next = b;
        self[c].prev = b;
    }

    /// Removes `b` from `a <-> b <-> c` to get `a <-> c`
    fn remove(&mut self, b: Cell) {
        let a = self[b].prev;
        let c = self[b].next;

        self[a].next = self[b].next;
        self[c].prev = self[b].prev;
    }

    /// Restores previously removed `b` to get `a <-> b <-> c`
    fn restore(&mut self, b: Cell) {
        let a = self[b].prev;
        let c = self[b].next;
        self[a].next = b;
        self[c].prev = b;
    }

    fn cursor(&self, head: Cell) -> Cursor {
        Cursor { head, curr: head }
    }
}

struct Cursor {
    head: Cell,
    curr: Cell,
}

impl Cursor {
    fn next(&mut self, list: &LinkedList) -> Option<Cell> {
        self.curr = list[self.curr].next;

        if self.curr == self.head {
            return None;
        }

        Some(self.curr)
    }

    fn prev(&mut self, list: &LinkedList) -> Option<Cell> {
        self.curr = list[self.curr].prev;

        if self.curr == self.head {
            return None;
        }

        Some(self.curr)
    }
}

#[derive(Debug)]
pub struct Matrix {
    // Auxilary map to get from cell to row, could be encoded more efficiently.
    row_ranges: Vec<Range<Cell>>,
    // SoA fields
    size: Vec<usize>,
    c: Vec<Cell>,
    x: LinkedList,
    y: LinkedList,
}

impl ops::Index<Cell> for Vec<usize> {
    type Output = usize;

    fn index(&self, index: Cell) -> &usize {
        &self[index.0]
    }
}

impl ops::IndexMut<Cell> for Vec<usize> {
    fn index_mut(&mut self, index: Cell) -> &mut usize {
        &mut self[index.0]
    }
}

impl ops::Index<Cell> for Vec<Cell> {
    type Output = Cell;

    fn index(&self, index: Cell) -> &Cell {
        &self[index.0]
    }
}

impl ops::IndexMut<Cell> for Vec<Cell> {
    fn index_mut(&mut self, index: Cell) -> &mut Cell {
        &mut self[index.0]
    }
}

impl Matrix {
    pub fn new(n_cols: usize) -> Self {
        let mut res = Self {
            row_ranges: Vec::new(),
            size: Vec::with_capacity(n_cols + 1),
            c: Vec::with_capacity(n_cols + 1),
            x: LinkedList::with_capacity(n_cols + 1),
            y: LinkedList::with_capacity(n_cols + 1),
        };

        assert_eq!(res.alloc_column(), Cell(0));

        for _ in 0..n_cols {
            res.add_column();
        }

        res
    }

    fn alloc(&mut self, c: Cell) -> Cell {
        self.c.push(c);

        let cell = self.x.alloc();
        assert_eq!(self.y.alloc(), cell);

        cell
    }

    fn alloc_column(&mut self) -> Cell {
        let cell = self.alloc(Cell(0));

        self.c[cell] = cell;
        self.size.push(0);

        cell
    }

    fn add_column(&mut self) {
        let new_col = self.alloc_column();
        self.x.insert(self.x[Cell(0)].prev, new_col);
    }

    pub fn add_row(&mut self, row: &[bool]) {
        assert_eq!(row.len(), self.size.len() - 1);

        let row_start = Cell(self.x.data.len());
        let mut c = Cell(0);
        let mut prev = None;

        for &is_filled in row {
            c = self.x[c].next;

            if is_filled {
                self.size[c] += 1;

                let new_cell = self.alloc(c);
                self.y.insert(self.y[c].prev, new_cell);

                if let Some(prev) = prev {
                    self.x.insert(prev, new_cell);
                }

                prev = Some(new_cell);
            }
        }

        let row_end = Cell(self.x.data.len());
        self.row_ranges.push(row_start..row_end);
    }

    fn row_of(&self, cell: Cell) -> usize {
        self.row_ranges
            .binary_search_by(|range| {
                if cell < range.start {
                    Ordering::Greater
                } else if range.start <= cell && cell < range.end {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            })
            .unwrap()
    }

    fn cover(&mut self, c: Cell) {
        self.x.remove(c);
        let mut i = self.y.cursor(c);

        while let Some(i) = i.next(&self.y) {
            let mut j = self.x.cursor(i);

            while let Some(j) = j.next(&self.x) {
                self.y.remove(j);
                self.size[self.c[j]] -= 1;
            }
        }
    }

    fn uncover(&mut self, c: Cell) {
        let mut i = self.y.cursor(c);

        while let Some(i) = i.prev(&self.y) {
            let mut j = self.x.cursor(i);

            while let Some(j) = j.prev(&self.x) {
                self.size[self.c[j]] += 1;
                self.y.restore(j);
            }
        }

        self.x.restore(c);
    }
}

// Reference: http://www.secmem.org/blog/2019/12/15/knuths-algorithm-x/
// Reference: https://github.com/matklad/dlx
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let mut sudoku = [[0; 16]; 16];
    let mut matrix = Matrix::new(1024);
    let mut data = Vec::new();

    let mut make_row = |i: usize, j: usize, k: usize| {
        let mut row = vec![false; 1024];
        row[i * 16 + j] = true;
        row[256 + i * 16 + k] = true;
        row[256 * 2 + j * 16 + k] = true;
        row[256 * 3 + (i / 4 * 4 + j / 4) * 16 + k] = true;

        matrix.add_row(&row);
        data.push((i, j, k));
    };

    for i in 0..16 {
        let mut s = scan.token::<String>();
        s = s.trim().to_string();
        let chars = s.chars();

        for (j, c) in chars.enumerate() {
            let num = if c == '-' { 0 } else { c as u8 - b'A' + 1 };
            sudoku[i][j] = num;

            if num > 0 {
                make_row(i, j, num as usize - 1);
            } else {
                for k in 0..16 {
                    make_row(i, j, k);
                }
            }
        }
    }

    let ret = solve(matrix);

    for val in ret[0].iter() {
        let (i, j, k) = data[*val];
        sudoku[i][j] = k as u8 + b'A';
    }

    for i in 0..16 {
        for j in 0..16 {
            write!(out, "{}", sudoku[i][j] as char).unwrap();
        }

        writeln!(out).unwrap();
    }
}
