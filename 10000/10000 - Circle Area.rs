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
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum State {
    None,
    Connect,
    Disconnect,
}

struct Circle {
    is_left_pos: bool,
    left_pos: i64,
    right_pos: i64,
    radius: i64,
}

impl Circle {
    fn new(is_left_pos: bool, left_pos: i64, right_pos: i64, radius: i64) -> Self {
        Self {
            is_left_pos,
            left_pos,
            right_pos,
            radius,
        }
    }

    fn get_position(&self) -> i64 {
        if self.is_left_pos {
            self.left_pos
        } else {
            self.right_pos
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut circles = Vec::new();

    for _ in 0..n {
        let (x, r) = (scan.token::<i64>(), scan.token::<i64>());

        circles.push(Circle::new(true, x - r, x + r, r));
        circles.push(Circle::new(false, x - r, x + r, r));
    }

    circles.sort_by(|a, b| {
        if a.get_position() != b.get_position() {
            a.get_position().cmp(&b.get_position())
        } else if a.is_left_pos != b.is_left_pos {
            a.is_left_pos.cmp(&b.is_left_pos)
        } else if a.radius != b.radius {
            b.radius.cmp(&a.radius)
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut num_area = 0;
    let mut stack = Vec::new();

    for i in 0..circles.len() {
        let circle = &circles[i];

        if !stack.is_empty() {
            let parent_state = stack.last_mut().unwrap();

            if *parent_state == State::None || *parent_state == State::Connect {
                *parent_state = if circle.get_position() == circles[i - 1].get_position() {
                    State::Connect
                } else {
                    State::Disconnect
                };
            }
        }

        if circle.is_left_pos {
            stack.push(State::None);
        } else {
            let state = stack.pop().unwrap();
            if state == State::Connect {
                num_area += 1;
            }
        }
    }

    writeln!(out, "{}", num_area + n + 1).unwrap();
}
