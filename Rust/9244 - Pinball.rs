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
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialOrd)]
struct Interval {
    left: Point,
    right: Point,
    direction: Direction,
    idx: usize,
}

impl Interval {
    fn new(idx: usize, p1: Point, p2: Point) -> Self {
        let left = if p1.x <= p2.x { p1 } else { p2 };
        let right = if p1.x <= p2.x { p2 } else { p1 };
        let direction = if left.y < right.y {
            Direction::Left
        } else {
            Direction::Right
        };

        Self {
            left,
            right,
            direction,
            idx,
        }
    }

    fn is_between(&self, p: &Point) -> bool {
        self.left.x <= p.x && p.x <= self.right.x
    }

    fn is_below(&self, p: &Point) -> bool {
        ((self.right.x - self.left.x) * (p.y - self.left.y)
            - (self.right.y - self.left.y) * (p.x - self.left.x))
            < 0
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Eq for Interval {}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.idx == other.idx {
            return std::cmp::Ordering::Equal;
        }

        if self.is_between(&other.left) {
            if self.is_below(&other.left) {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }

        if other.is_between(&self.left) {
            if other.is_below(&self.left) {
                return std::cmp::Ordering::Greater;
            } else {
                return std::cmp::Ordering::Less;
            }
        }

        if self.idx < other.idx {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum EventType {
    RightUp,
    RightDown,
    LeftUp,
    LeftDown,
}

struct Event {
    event_type: EventType,
    x: i64,
    num: i64,
}

impl Event {
    fn new(interval: Interval, side: Direction) -> Self {
        Self {
            event_type: match side {
                Direction::Left => match interval.direction {
                    Direction::Left => EventType::LeftDown,
                    Direction::Right => EventType::LeftUp,
                },
                Direction::Right => match interval.direction {
                    Direction::Left => EventType::RightUp,
                    Direction::Right => EventType::RightDown,
                },
            },
            x: match side {
                Direction::Left => interval.left.x,
                Direction::Right => interval.right.x,
            },
            num: interval.idx as i64,
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut input = Vec::new();
    let mut events = Vec::new();

    for i in 0..n {
        let (x1, y1, x2, y2) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
        let interval = Interval::new(i, Point::new(x1, y1), Point::new(x2, y2));

        input.push(interval);
        events.push(Event::new(interval, Direction::Left));
        events.push(Event::new(interval, Direction::Right));
    }

    let x0 = scan.token::<i64>();
    let interval = Interval::new(n, Point::new(x0 - 1, 2_000_001), Point::new(x0, 2_000_000));

    input.push(interval);
    events.push(Event::new(interval, Direction::Left));
    events.push(Event::new(interval, Direction::Right));

    events.sort_by(|a, b| {
        if a.x != b.x {
            a.x.cmp(&b.x)
        } else {
            b.event_type.cmp(&a.event_type)
        }
    });

    let mut sweep = BTreeSet::new();
    let mut succ: Vec<i64> = vec![-2; n + 1];

    for event in events.iter() {
        println!("{}", event.num);
        let event_num = event.num as usize;

        match event.event_type {
            EventType::LeftUp => {
                sweep.insert(input[event_num]);
            }
            EventType::RightUp => {
                sweep.remove(&input[event_num]);
            }
            EventType::LeftDown => {
                sweep.insert(input[event_num]);

                if let Some(val) = sweep.iter().last() {
                    if val == &input[event_num] {
                        succ[event_num] = -1;
                    } else {
                        succ[event_num] = val.idx as i64;
                    }
                }
            }
            EventType::RightDown => {
                if let Some(val) = sweep.iter().last() {
                    if val == &input[event_num] {
                        succ[event_num] = -1;
                    } else {
                        succ[event_num] = val.idx as i64;
                    }
                }

                sweep.remove(&input[event_num]);
            }
        }
    }

    let mut query = n;

    while succ[query] >= 0 {
        query = succ[query] as usize;
    }

    let x_pos = if input[query].direction == Direction::Left {
        input[query].left.x
    } else {
        input[query].right.x
    };

    writeln!(out, "{}", x_pos).unwrap();
}
ㅌ