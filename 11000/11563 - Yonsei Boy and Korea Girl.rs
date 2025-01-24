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

    pub fn line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Segment {
    p1: Point,
    p2: Point,
}

fn dist_point_to_point(a: &Point, b: &Point) -> f64 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;

    (dx * dx + dy * dy).sqrt()
}

fn dist_point_to_segment(p: &Point, seg: &Segment) -> f64 {
    let Segment { p1, p2 } = seg;
    let vx = p2.x - p1.x;
    let vy = p2.y - p1.y;
    let wx = p.x - p1.x;
    let wy = p.y - p1.y;

    let seg_len2 = vx * vx + vy * vy;

    if seg_len2 < 1e-15 {
        return dist_point_to_point(p, p1);
    }

    let t = (wx * vx + wy * vy) / seg_len2;

    if t < 0.0 {
        dist_point_to_point(p, p1)
    } else if t > 1.0 {
        dist_point_to_point(p, p2)
    } else {
        let proj_x = p1.x + t * vx;
        let proj_y = p1.y + t * vy;

        dist_point_to_point(
            p,
            &Point {
                x: proj_x,
                y: proj_y,
            },
        )
    }
}

fn dist_segment_to_segment(s1: &Segment, s2: &Segment) -> f64 {
    let d1 = dist_point_to_segment(&s2.p1, s1);
    let d2 = dist_point_to_segment(&s2.p2, s1);
    let d3 = dist_point_to_segment(&s1.p1, s2);
    let d4 = dist_point_to_segment(&s1.p2, s2);

    let mut ret = d1.min(d2).min(d3).min(d4);

    let (p1, p2) = (&s1.p1, &s1.p2);
    let (p3, p4) = (&s2.p1, &s2.p2);

    let v1_x = p2.x - p1.x;
    let v1_y = p2.y - p1.y;
    let v2_x = p4.x - p3.x;
    let v2_y = p4.y - p3.y;

    let w0_x = p1.x - p3.x;
    let w0_y = p1.y - p3.y;

    let a = v1_x * v1_x + v1_y * v1_y;
    let b = v1_x * v2_x + v1_y * v2_y;
    let c = v2_x * v2_x + v2_y * v2_y;
    let d = v1_x * w0_x + v1_y * w0_y;
    let e = v2_x * w0_x + v2_y * w0_y;

    let denom = a * c - b * b;

    if denom.abs() > 1e-15 {
        let t_clamped = (b * e - c * d) / denom;
        let u_clamped = (a * e - b * d) / denom;

        if (0.0..=1.0).contains(&t_clamped) && (0.0..=1.0).contains(&u_clamped) {
            let cx_s1 = p1.x + t_clamped * v1_x;
            let cy_s1 = p1.y + t_clamped * v1_y;
            let cx_s2 = p3.x + u_clamped * v2_x;
            let cy_s2 = p3.y + u_clamped * v2_y;

            let dist =
                dist_point_to_point(&Point { x: cx_s1, y: cy_s1 }, &Point { x: cx_s2, y: cy_s2 });

            ret = ret.min(dist);
        }
    }

    ret
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m) = (scan.token::<usize>(), scan.token::<usize>());
    let mut roads_sinchon = Vec::with_capacity(n);
    let mut roads_anam = Vec::with_capacity(m);

    for _ in 0..n {
        let (xs, ys, xe, ye) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        roads_sinchon.push(Segment {
            p1: Point { x: xs, y: ys },
            p2: Point { x: xe, y: ye },
        });
    }

    for _ in 0..m {
        let (xs, ys, xe, ye) = (
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
            scan.token::<f64>(),
        );
        roads_anam.push(Segment {
            p1: Point { x: xs, y: ys },
            p2: Point { x: xe, y: ye },
        });
    }

    let mut ret = f64::MAX;

    for i in 0..n {
        for j in 0..m {
            ret = ret.min(dist_segment_to_segment(&roads_sinchon[i], &roads_anam[j]));
        }
    }

    writeln!(out, "{:.10}", ret).unwrap();
}
