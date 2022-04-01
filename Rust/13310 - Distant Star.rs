use io::Write;
use std::{cmp, io, ops::Sub, str};

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

#[derive(Clone)]
struct Star {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Star {
    fn new(x: i64, y: i64, dx: i64, dy: i64) -> Self {
        Self { x, y, dx, dy }
    }
}

impl Sub for Star {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            dx: 0,
            dy: 0,
        }
    }
}

fn calculate_ccw(p1: Star, p2: Star, p3: Star) -> i64 {
    let (x1, y1) = (p1.x, p1.y);
    let (x2, y2) = (p2.x, p2.y);
    let (x3, y3) = (p3.x, p3.y);

    let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

fn next_to_top(stack: &mut Vec<Star>) -> Star {
    let top = stack.pop().unwrap();
    let next = stack.pop().unwrap();

    stack.push(next.clone());
    stack.push(top);

    next
}

fn get_dist(p1: &Star, p2: &Star) -> i64 {
    (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y)
}

// Reference: https://stackoverflow.com/questions/28294735/how-to-swap-the-elements-of-an-array-slice-or-vec
fn swap<T>(x: &mut [T], i: usize, j: usize) {
    let (lo, hi) = match i.cmp(&j) {
        // no swapping necessary
        cmp::Ordering::Equal => return,

        // get the smallest and largest of the two indices
        cmp::Ordering::Less => (i, j),
        cmp::Ordering::Greater => (j, i),
    };

    let (init, tail) = x.split_at_mut(hi);
    std::mem::swap(&mut init[lo], &mut tail[0]);
}

fn search_ternary(stars: &Vec<Star>, time: i64, n: usize) -> i64 {
    let mut new_stars: Vec<Star> = stars
        .iter()
        .map(|star| Star::new(star.x + star.dx * time, star.y + star.dy * time, 0, 0))
        .collect();

    let min_star = new_stars.iter().min_by_key(|star| star.x).unwrap();
    let min_idx = new_stars
        .iter()
        .position(|star| star.x == min_star.x && star.y == min_star.y)
        .unwrap();
    swap(&mut new_stars, 0, min_idx);

    let first_point = new_stars.remove(0);
    new_stars.sort_by(|a, b| {
        let ccw = calculate_ccw(first_point.clone(), a.clone(), b.clone());

        if ccw != 0 {
            if ccw > 0 {
                return cmp::Ordering::Less;
            } else {
                return cmp::Ordering::Greater;
            }
        }

        if get_dist(&first_point, &a) < get_dist(&first_point, &b) {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    });
    new_stars.insert(0, first_point);

    let mut stack = Vec::new();
    stack.push(new_stars[0].clone());
    stack.push(new_stars[1].clone());

    for i in 2..n {
        while stack.len() >= 2
            && calculate_ccw(
                stack.last().unwrap().clone(),
                next_to_top(&mut stack),
                new_stars[i].clone(),
            ) >= 0
        {
            stack.pop();
        }

        stack.push(new_stars[i].clone());
    }

    let mut convex_hull = vec![Star::new(0, 0, 0, 0); stack.len()];
    let mut index = stack.len() - 1;

    while !stack.is_empty() {
        convex_hull[index] = stack.pop().unwrap();
        index -= 1;
    }

    let mut max_dist = 0;
    let mut c = 1;

    for a in 0..convex_hull.len() {
        let b = (a + 1) % convex_hull.len();

        loop {
            let d = (c + 1) % convex_hull.len();

            let zero = Star::new(0, 0, 0, 0);
            let ab = convex_hull[b].clone() - convex_hull[a].clone();
            let cd = convex_hull[d].clone() - convex_hull[c].clone();

            if calculate_ccw(zero, ab, cd) > 0 {
                c = d;
            } else {
                break;
            }
        }

        let dist = get_dist(&convex_hull[a], &convex_hull[c]);
        if dist > max_dist {
            max_dist = dist;
        }
    }

    max_dist
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, t) = (scan.token::<usize>(), scan.token::<i64>());
    let mut stars = Vec::new();

    for _ in 0..n {
        stars.push(Star::new(
            scan.token(),
            scan.token(),
            scan.token(),
            scan.token(),
        ));
    }

    let mut left = 0;
    let mut right = t;

    while left + 3 < right {
        let mid_left = (left + left + right) / 3;
        let mid_right = (left + right + right) / 3;

        if search_ternary(&stars, mid_left, n) > search_ternary(&stars, mid_right, n) {
            left = mid_left;
        } else {
            right = mid_right;
        }
    }

    let mut ans = i64::MAX;
    let mut idx = left;

    for i in left..=right {
        let ret = search_ternary(&mut stars, i, n);

        if ans > ret {
            ans = ret;
            idx = i;
        }
    }

    writeln!(out, "{}", idx).unwrap();
    writeln!(out, "{}", ans).unwrap();
}
