use io::Write;
use std::{cmp::Ordering, collections::BTreeSet, io, str};

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

const EPS: f64 = 1e-9;
static mut SWEEP_LINE_X: i64 = -100_000_000;

#[derive(Clone, Debug)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}

impl Circle {
    fn y(&self, sweep_line_x: i64, up_side: bool) -> f64 {
        let b = -2 * self.y;
        let c =
            self.y * self.y + (sweep_line_x - self.x) * (sweep_line_x - self.x) - self.r * self.r;
        let delta = b * b - 4 * c;
        let delta_sqrt = (delta as f64).sqrt();

        if up_side {
            (-b as f64 + delta_sqrt) / 2.0
        } else {
            (-b as f64 - delta_sqrt) / 2.0
        }
    }
}

#[derive(Clone, Debug)]
struct Curve {
    up_side: bool,
    circle: Circle,
}

impl Curve {
    fn y(&self, sweep_line_x: i64) -> f64 {
        self.circle.y(sweep_line_x, self.up_side)
    }
}

#[derive(Clone, Debug)]
struct Interval {
    id: usize,
    up: Curve,
    down: Curve,
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        !(*self < *other) && !(*other < *self)
    }
}

impl Eq for Interval {}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let sx = unsafe { sweep_line_x() };
        let down_self = self.down.y(sx);
        let down_other = other.down.y(sx);

        if (down_self - down_other).abs() > EPS {
            return down_self.partial_cmp(&down_other);
        }

        let up_self = self.up.y(sx);
        let up_other = other.up.y(sx);

        up_self.partial_cmp(&up_other)
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

unsafe fn sweep_line_x() -> i64 {
    SWEEP_LINE_X
}

unsafe fn set_sweep_line_x(val: i64) {
    SWEEP_LINE_X = val;
}

fn process_dfs(graph: &Vec<Vec<usize>>, dp: &mut Vec<Vec<i64>>, node: usize, n: usize) {
    dp[node][0] = 0;
    dp[node][1] = 1;

    for &next in graph[node].iter() {
        process_dfs(graph, dp, next, n);

        if next > n {
            dp[node][0] += dp[next][0];
            dp[node][1] += dp[next][1] - 1;
        } else {
            dp[node][0] += dp[next][0].max(dp[next][1]);
            dp[node][1] += dp[next][0];
        }
    }
}

// Reference: https://codeforces.com/blog/entry/98782
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, q) = (scan.token::<usize>(), scan.token::<usize>());
    let mut circles = Vec::with_capacity(n + q + 1);
    let mut events = Vec::with_capacity(2 * (n + q));

    circles.push(Circle {
        x: 0,
        y: 0,
        r: 1_000_000_000,
    });

    for i in 1..=n + q {
        let (x, y, r) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        circles.push(Circle { x, y, r });
        events.push((x - r, i));
        events.push((x + r, i));
    }

    events.sort_by_key(|e| e.0);

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n + q + 1];
    let mut intervals = BTreeSet::new();

    intervals.insert(Interval {
        id: 0,
        up: Curve {
            up_side: true,
            circle: circles[0].clone(),
        },
        down: Curve {
            up_side: false,
            circle: circles[0].clone(),
        },
    });

    for (sw_x, circle_id) in events {
        unsafe { set_sweep_line_x(sw_x) };

        let x = circles[circle_id].x;
        let r = circles[circle_id].r;

        let inside = Interval {
            id: circle_id,
            up: Curve {
                up_side: true,
                circle: circles[circle_id].clone(),
            },
            down: Curve {
                up_side: false,
                circle: circles[circle_id].clone(),
            },
        };

        if sw_x == x - r {
            let mut iter = intervals.range(..inside.clone()).rev();

            if let Some(interval_parent) = iter.next() {
                let parent = interval_parent.clone();

                drop(iter);

                intervals.remove(&parent);
                graph[parent.id].push(circle_id);

                let mut interval_up = parent.clone();
                let mut interval_down = parent.clone();

                interval_up.down = Curve {
                    up_side: true,
                    circle: circles[circle_id].clone(),
                };

                interval_down.up = Curve {
                    up_side: false,
                    circle: circles[circle_id].clone(),
                };

                intervals.insert(interval_up);
                intervals.insert(interval_down);
                intervals.insert(inside);
            }
        } else {
            let mut iter = intervals.range(inside.clone()..);
            let mid = iter.next().cloned();

            drop(iter);

            if let Some(mid) = mid {
                let mut iter_prev = intervals.range(..mid.clone()).rev();
                let joined = iter_prev.next().cloned();

                drop(iter_prev);

                if let Some(joined) = joined {
                    let mut iter_next = intervals.range(mid.clone()..).skip(1);
                    let val_next = iter_next.next().cloned();

                    drop(iter_next);

                    if let Some(val_next) = val_next {
                        let mut joined_new = joined.clone();
                        joined_new.up = val_next.up.clone();

                        intervals.remove(&joined);
                        intervals.remove(&mid);
                        intervals.remove(&val_next);
                        intervals.insert(joined_new);
                    }
                }
            }
        }
    }

    let mut dp = vec![vec![0; 2]; n + q + 1];

    process_dfs(&graph, &mut dp, 0, n);

    for i in n + 1..=n + q {
        writeln!(out, "{}", dp[i][0].max(dp[i][1])).unwrap();
    }
}
