use io::Write;
use std::{cmp::Ordering, io, str};
use Ordering::{Equal, Less, Greater};

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

#[derive(Clone, Debug)]
struct Disc {
    center: (f64, f64),
    radius: f64,
}

impl Disc {
    pub fn new(center: (f64, f64), radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn exist_common_support_line(&self, other: &Disc) -> bool {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);
        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);

        square > (r1 - r2).powi(2)
    }

    pub fn get_leftmost_point(&self) -> (f64, f64) {
        (self.center.0 - self.radius, self.center.1)
    }

    pub fn compute_common_support_line_to(&self, other: &Disc) -> Edge {
        let (x1, y1, r1) = (self.center.0, self.center.1, self.radius);
        let (x2, y2, r2) = (other.center.0, other.center.1, other.radius);

        let square = (x1 - x2).powi(2) + (y1 - y2).powi(2);
        let d = square.sqrt();
        let vx = (x2 - x1) / d;
        let vy = (y2 - y1) / d;

        let c = (r1 - r2) / d;
        let h = (1.0 - c.powi(2)).sqrt();

        let nx = vx * c - vy * h;
        let ny = vx * h + vy * c;

        Edge {
            start: (x1 + r1 * nx, y1 + r1 * ny),
            end: (x2 + r2 * nx, y2 + r2 * ny),
        }
    }
}

impl PartialEq for Disc {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center && self.radius == other.radius
    }
}

impl PartialOrd for Disc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.radius < other.radius {
            Some(Less)
        } else if self.radius > other.radius {
            Some(Greater)
        } else {
            Some(Equal)
        }
    }
}

#[derive(Debug)]
struct Edge {
    start: (f64, f64),
    end: (f64, f64),
}

#[derive(Debug)]
struct ConvexHull {
    discs: Vec<Disc>,
    edges: Vec<Edge>,
}

impl ConvexHull {
    fn length(&self) -> f64 {
        0.0
    }
}

#[derive(Default, Debug)]
struct ConvexHullAlgorithm {
    discs: Vec<Disc>,
}

impl ConvexHullAlgorithm {
    pub fn add_disc(&mut self, disc: Disc) {
        self.discs.push(disc);
    }

    pub fn find(&self) -> ConvexHull {
        if self.discs.len() == 1 {
            return ConvexHull {
                discs: self.discs.clone(),
                edges: Vec::new(),
            };
        }

        if self.discs.len() == 2 {
            self.merge_two_discs(0, 1);
        }

        ConvexHull {
            discs: self.discs.clone(),
            edges: Vec::new(),
        }
    }
}

impl ConvexHullAlgorithm {
    fn merge_two_discs(&self, index1: usize, index2: usize) -> ConvexHull {
        let x = self.discs[index1].clone();
        let y = self.discs[index2].clone();

        if !x.exist_common_support_line(&y) {
            if x.partial_cmp(&y) == Some(Less) {
                return ConvexHull {
                    discs: vec![x],
                    edges: Vec::new(),
                };
            } else {
                return ConvexHull {
                    discs: vec![y],
                    edges: Vec::new(),
                };
            }
        }

        let p1 = x.get_leftmost_point().0;
        let p2 = y.get_leftmost_point().0;
        let e1 = x.compute_common_support_line_to(&y);
        let e2 = y.compute_common_support_line_to(&x);

        if p1 < p2 {
            ConvexHull {
                discs: vec![x.clone(), y, x],
                edges: vec![e1, e2],
            }
        } else if p1 > p2 {
            ConvexHull {
                discs: vec![y.clone(), x, y],
                edges: vec![e2, e1],
            }
        } else {
            if x.center.1 < y.center.1 {
                ConvexHull {
                    discs: vec![x.clone(), y, x],
                    edges: vec![e1, e2],
                }
            } else {
                ConvexHull {
                    discs: vec![y.clone(), x, y],
                    edges: vec![e2, e1],
                }
            }
        }
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let c = scan.token::<i64>();

    for _ in 0..c {
        let n = scan.token::<i64>();
        let mut convex_hull_algorithm = ConvexHullAlgorithm::default();

        for _ in 0..n {
            let (x, y, r) = (scan.token::<f64>(), scan.token::<f64>(), scan.token::<f64>());
            convex_hull_algorithm.add_disc(Disc::new((x, y), r));
        }

        writeln!(out, "{:?}", convex_hull_algorithm.find().length()).unwrap();
    }
}
