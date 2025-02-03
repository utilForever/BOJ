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

#[derive(Eq, PartialEq)]
struct Event {
    x: i64,
    y: i64,
    r: i64,
    dir: i64,
}

impl Event {
    fn new(x: i64, y: i64, r: i64, dir: i64) -> Self {
        Self { x, y, r, dir }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.x + self.dir * self.r;
        let rhs = other.x + other.dir * other.r;

        if lhs != rhs {
            return Some(lhs.cmp(&rhs));
        }

        if self.y != other.y {
            return Some(self.y.cmp(&other.y));
        }

        if self.dir != other.dir {
            return Some(other.dir.cmp(&self.dir));
        }

        Some(other.r.cmp(&self.r))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

static mut T: i64 = 0;

#[derive(Clone, Copy, Eq, PartialEq)]
struct CircleEvent {
    x: i64,
    y: i64,
    r: i64,
    dir: i64,
}

impl CircleEvent {
    fn new(x: i64, y: i64, r: i64, dir: i64) -> Self {
        Self { x, y, r, dir }
    }
}

impl PartialOrd for CircleEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let lhs = self.y as f64
            + self.dir as f64
                * ((self.r * self.r - (self.x - unsafe { T }) * (self.x - unsafe { T })) as f64)
                    .sqrt();
        let rhs = other.y as f64
            + other.dir as f64
                * ((other.r * other.r - (other.x - unsafe { T }) * (other.x - unsafe { T }))
                    as f64)
                    .sqrt();

        if lhs + 1e-9 < rhs {
            return Some(std::cmp::Ordering::Less);
        }

        if lhs > rhs + 1e-9 {
            return Some(std::cmp::Ordering::Greater);
        }

        if self.dir != other.dir {
            return Some(self.dir.cmp(&other.dir));
        }

        Some(other.r.cmp(&self.r))
    }
}

impl Ord for CircleEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut events = Vec::with_capacity(n * 2);

    for _ in 0..n {
        let (x, y, r) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        events.push(Event::new(x, y, r, -1));
        events.push(Event::new(x, y, r, 1));
    }

    events.sort();

    let mut set = BTreeSet::new();
    set.insert(CircleEvent::new(0, -2_000_000_000, 1_000_000_000, 1));

    let mut ret = 0.0;

    for event in events {
        unsafe {
            T = event.x + event.dir * event.r;
        }

        if event.dir == -1 {
            let point = CircleEvent::new(event.x, event.y, event.r, -1);
            let range = *set.range(..point).next_back().unwrap();

            if range.dir == 1 {
                ret += std::f64::consts::PI * (event.r as f64).powi(2);

                set.insert(CircleEvent::new(event.x, event.y, event.r, -1));
                set.insert(CircleEvent::new(event.x, event.y, event.r, 1));
            }
        } else {
            set.remove(&CircleEvent::new(event.x, event.y, event.r, -1));
            set.remove(&CircleEvent::new(event.x, event.y, event.r, 1));
        }
    }

    writeln!(out, "{:.12}", ret).unwrap();
}
