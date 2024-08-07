use io::Write;
use std::{collections::BTreeSet, io, str};

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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

pub struct Combinations<T> {
    elements: Vec<T>,
    positions: Vec<usize>,
    all_sizes: bool,
    done: bool,
}

fn iterable_to_sorted_set<T: Ord + Clone>(elements: impl IntoIterator<Item = T>) -> Vec<T> {
    elements
        .into_iter()
        .collect::<BTreeSet<T>>()
        .into_iter()
        .collect::<Vec<T>>()
}

impl<T: Ord + Clone> Combinations<T> {
    pub fn all(elements: impl IntoIterator<Item = T>) -> Self {
        Combinations {
            elements: iterable_to_sorted_set(elements),
            positions: Vec::new(),
            all_sizes: true,
            done: false,
        }
    }

    pub fn of_size(elements: impl IntoIterator<Item = T>, size: usize) -> Self {
        Combinations {
            elements: iterable_to_sorted_set(elements),
            positions: (0..size).collect(),
            all_sizes: false,
            done: false,
        }
    }

    fn move_to_next_set_size(&mut self) -> bool {
        if self.positions.len() >= self.elements.len() {
            return false;
        }

        self.positions
            .iter_mut()
            .enumerate()
            .for_each(|(index, pos)| *pos = index);
        self.positions.push(self.positions.len());

        true
    }

    fn move_to_next_position(&mut self) -> bool {
        if self.elements.len() == 0 {
            return false;
        }

        let length = self.positions.len();

        for index in (0..self.positions.len()).rev() {
            let cur_position = *self.positions.get(index).unwrap();

            if cur_position >= self.elements.len() - 1 {
                continue;
            }

            if index == length - 1 || cur_position < self.positions.get(index + 1).unwrap() - 1 {
                let mut next_position = cur_position + 1;

                *self.positions.get_mut(index).unwrap() = next_position;

                for i in index + 1..length {
                    next_position += 1;
                    *self.positions.get_mut(i).unwrap() = next_position;
                }

                return true;
            }
        }

        false
    }

    fn get_current_combination(&mut self) -> Option<Vec<T>> {
        if self.done || self.positions.len() > self.elements.len() {
            return None;
        }

        Some(
            self.positions
                .iter()
                .map(|p| self.elements.get(*p).unwrap().clone())
                .collect::<Vec<T>>(),
        )
    }
}

impl<T: Ord + Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let combo = self.get_current_combination();

        if self.move_to_next_position() == false {
            if self.all_sizes == false || self.move_to_next_set_size() == false {
                self.done = true;
            }
        }

        combo
    }
}

#[derive(Default, Clone, Copy)]
struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Reference: https://github.com/olivercalder/combinatorial
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut points = vec![Vector2::default(); n];

        for i in 0..n {
            points[i] = Vector2::new(scan.token::<f64>(), scan.token::<f64>());
        }

        let combinations = Combinations::of_size((0..n).collect::<Vec<usize>>(), n / 2);
        let mut flag = vec![false; n];
        let mut ret = f64::MAX;

        for idxes in combinations {
            let mut sum = Vector2::default();

            flag.fill(false);

            for idx in idxes {
                flag[idx] = true;
            }

            for i in 0..n {
                if flag[i] {
                    sum = sum + points[i];
                } else {
                    sum = sum - points[i];
                }
            }

            ret = ret.min(sum.x * sum.x + sum.y * sum.y);
        }

        writeln!(out, "{:.9}", ret.sqrt()).unwrap();
    }
}
