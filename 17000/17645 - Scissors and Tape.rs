use io::Write;
use std::{
    io,
    ops::{Add, Div, Mul, Sub},
    str, vec,
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

const EPS: f64 = 1e-9;

#[derive(Debug, Default, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[inline(always)]
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline(always)]
    fn cross(&self, other: &Point) -> f64 {
        self.x * other.y - self.y * other.x
    }

    #[inline(always)]
    fn ccw(p1: Point, p2: Point, p3: Point) -> f64 {
        (p2 - p1).cross(&(p3 - p1))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

type Polygon = Vec<Point>;

fn polygon_area(polygon: &Polygon) -> f64 {
    let n = polygon.len();
    let mut area = 0.0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += polygon[i].cross(&polygon[j]);
    }

    area.abs() / 2.0
}

fn side_lengths(polygon: &Polygon) -> Vec<f64> {
    let n = polygon.len();
    let mut lengths = Vec::with_capacity(n);

    for i in 0..n {
        lengths.push((polygon[(i + 1) % n] - polygon[i]).len());
    }

    lengths
}

#[derive(Debug, Clone)]
struct Shape {
    id: i64,
    polygon: Polygon,
}

impl Shape {
    fn new(id: i64, polygon: Polygon) -> Self {
        Self { id, polygon }
    }
}

mod approx {
    use crate::EPS;

    fn zero(x: f64) -> bool {
        x.abs() <= EPS
    }

    pub(crate) fn eq(a: f64, b: f64) -> bool {
        if zero(a - b) {
            return true;
        }

        let left = a * (1.0 - EPS);
        let right = a * (1.0 + EPS);

        left.min(right) <= b && b <= left.max(right)
    }

    pub(crate) fn eq_vec(a: &Vec<f64>, b: &Vec<f64>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        for i in 0..a.len() {
            if !eq(a[i], b[i]) {
                return false;
            }
        }

        true
    }

    pub(crate) fn pos(x: f64) -> bool {
        x > EPS
    }

    pub(crate) fn neg(x: f64) -> bool {
        x < -EPS
    }
}

mod triangulation {
    use crate::{approx, Point, Polygon, EPS};

    fn in_triangle(a: Point, b: Point, c: Point, p: Point) -> bool {
        approx::pos(Point::ccw(a, b, p))
            && approx::pos(Point::ccw(b, c, p))
            && approx::pos(Point::ccw(c, a, p))
    }

    fn is_ear(polygon: &Polygon, a: Point, b: Point, c: Point) -> bool {
        if !approx::pos(Point::ccw(a, b, c)) {
            return false;
        }

        for &p in polygon.iter() {
            if (p.x - a.x).abs() <= EPS && (p.y - a.y).abs() <= EPS {
                continue;
            }

            if (p.x - b.x).abs() <= EPS && (p.y - b.y).abs() <= EPS {
                continue;
            }

            if (p.x - c.x).abs() <= EPS && (p.y - c.y).abs() <= EPS {
                continue;
            }

            if in_triangle(a, b, c, p) {
                return false;
            }
        }

        true
    }

    pub(crate) fn triangulate_by_ear_clipping(polygon: &Polygon) -> Vec<Polygon> {
        let n = polygon.len();

        if n == 3 {
            return vec![polygon.clone()];
        }

        for i in 0..n {
            let a = polygon[i];
            let b = polygon[(i + 1) % n];
            let c = polygon[(i + 2) % n];

            if !is_ear(polygon, a, b, c) {
                continue;
            }

            let mut polygon_reduced = polygon.clone();
            polygon_reduced.remove((i + 1) % n);

            let mut pieces = triangulate_by_ear_clipping(&polygon_reduced);
            pieces.push(vec![a, b, c]);
            return pieces;
        }

        unreachable!("Triangulation failed");
    }
}

enum OpType {
    Scissors(i64, Vec<Polygon>),           // (id, pieces)
    Tape(Vec<i64>, Vec<Polygon>, Polygon), // (ids, placed, result)
}

struct OpManager {
    next_id: i64,
    operations: Vec<OpType>,
}

impl OpManager {
    fn new() -> Self {
        Self {
            next_id: 1,
            operations: Vec::new(),
        }
    }

    fn next_id(&mut self) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn add_scissors(&mut self, id: i64, pieces: &Vec<Polygon>) -> Vec<Shape> {
        let mut shapes = Vec::with_capacity(pieces.len());

        for piece in pieces.iter() {
            let id_new = self.next_id();
            shapes.push(Shape::new(id_new, piece.clone()));
        }

        self.operations.push(OpType::Scissors(id, pieces.clone()));
        shapes
    }

    fn add_tape(&mut self, id: &Vec<i64>, placed: &Vec<Polygon>, result: &Polygon) -> Shape {
        self.operations
            .push(OpType::Tape(id.clone(), placed.clone(), result.clone()));

        let id = self.next_id();
        Shape::new(id, result.clone())
    }

    fn add_scissors_and_tape(
        &mut self,
        id: i64,
        pieces_cut: &Vec<Polygon>,
        pieces_placed: &Vec<Polygon>,
        result: &Polygon,
    ) -> Shape {
        let produced = self.add_scissors(id, pieces_cut);
        let ids = produced.iter().map(|s| s.id).collect::<Vec<_>>();
        self.add_tape(&ids, pieces_placed, result)
    }
}

// Solver for Wallace-Bolyai-Gerwien theorem
// Reference: https://en.wikipedia.org/wiki/Wallace%E2%80%93Bolyai%E2%80%93Gerwien_theorem
// Reference: https://jjycjnmath.tistory.com/499
struct WBGSolver {
    source: Polygon,
    target: Polygon,
    manager: OpManager,
}

impl WBGSolver {
    fn new(source: Polygon, target: Polygon) -> Self {
        Self {
            source,
            target,
            manager: OpManager::new(),
        }
    }

    fn solve(mut self) -> Vec<OpType> {
        let shape_source = Shape::new(0, self.source.clone());

        // Step 1: Cut the source polygon into triangles
        let triangles_source = self.cut_into_triangles(&shape_source);

        // Step 2: Reassemble triangles into a square
        let area = polygon_area(&self.source);
        let side = area.sqrt();

        let triangles_area = triangles_source
            .iter()
            .map(|t| polygon_area(&t.polygon))
            .collect::<Vec<_>>();
        let widths_strip = self.compute_widths_strip(&triangles_area, side);

        let mut rectangles_strip = Vec::with_capacity(triangles_source.len());

        for (triangle, width) in triangles_source.into_iter().zip(widths_strip.into_iter()) {
            let rect = self.triangle_to_rectangle_strip(triangle, width, side);
            rectangles_strip.push(rect);
        }

        let square = self.tape_rectangles_strip_into_square(&rectangles_strip, side);

        // Step 3: Triangulation of the target polygon
        let mut target_triangles = triangulation::triangulate_by_ear_clipping(&self.target);
        let target_areas = target_triangles
            .iter()
            .map(|t| polygon_area(t))
            .collect::<Vec<_>>();
        let target_widths = self.compute_widths_strip(&target_areas, side);

        // Step 4: Cut the square into rectangles to match the target triangles's area
        let pieces_rectangles = self.cut_square_into_rectangles(square, &target_widths, side);

        // // Step 5: Convert each rectangle into the corresponding canonical triangle (equivalent to the target triangle)
        let mut triangles_canonical = Vec::with_capacity(pieces_rectangles.len());

        for (rectangle, triangle) in pieces_rectangles.into_iter().zip(target_triangles.iter()) {
            let triangle_canonical = self.rectangle_to_canonical_triangle(rectangle, triangle);
            triangles_canonical.push(triangle_canonical);
        }

        // Step 6: Move canonical triangles to the target triangulation location
        //         and tape them to create the target polygon
        let _ =
            self.tape_triangles_into_target_polygon(&triangles_canonical, &mut target_triangles);

        self.manager.operations
    }

    fn cut_into_triangles(&mut self, shape: &Shape) -> Vec<Shape> {
        let triangles = triangulation::triangulate_by_ear_clipping(&shape.polygon);
        self.manager.add_scissors(shape.id, &triangles)
    }

    fn compute_widths_strip(&self, areas: &Vec<f64>, side: f64) -> Vec<f64> {
        let n = areas.len();
        let mut widths = vec![0.0; n];
        let mut sum = 0.0;

        for i in 0..n {
            if i == n - 1 {
                widths[i] = (side - sum).max(0.0);
            } else {
                widths[i] = areas[i] / side;
                sum += widths[i];
            }
        }

        widths
    }

    fn resize_rectangle(
        &mut self,
        mut rectangle: Shape,
        target_width: f64,
        target_height: f64,
    ) -> Shape {
        let target = Self::aabb_rectangle(target_width, target_height, 0.0, 0.0);

        loop {
            let (width, height) = (rectangle.polygon[2].x, rectangle.polygon[2].y);

            if approx::eq(width, target_width) && approx::eq(height, target_height) {
                return rectangle;
            }

            if !approx::neg(width - 2.0 * target_width) {
                let cut = vec![
                    Self::aabb_rectangle(width / 2.0, height, 0.0, 0.0),
                    Self::aabb_rectangle(width / 2.0, height, width / 2.0, 0.0),
                ];
                let placed = vec![
                    Self::aabb_rectangle(width / 2.0, height, 0.0, 0.0),
                    Self::aabb_rectangle(width / 2.0, height, 0.0, height),
                ];

                let ret = Self::aabb_rectangle(width / 2.0, height * 2.0, 0.0, 0.0);
                rectangle = self
                    .manager
                    .add_scissors_and_tape(rectangle.id, &cut, &placed, &ret);
                continue;
            }

            if !approx::neg(height - 2.0 * target_height) {
                let cut = vec![
                    Self::aabb_rectangle(width, height / 2.0, 0.0, 0.0),
                    Self::aabb_rectangle(width, height / 2.0, 0.0, height / 2.0),
                ];
                let placed = vec![
                    Self::aabb_rectangle(width, height / 2.0, 0.0, 0.0),
                    Self::aabb_rectangle(width, height / 2.0, width, 0.0),
                ];

                let ret = Self::aabb_rectangle(width * 2.0, height / 2.0, 0.0, 0.0);
                rectangle = self
                    .manager
                    .add_scissors_and_tape(rectangle.id, &cut, &placed, &ret);
                continue;
            }

            if approx::pos(width - target_width) {
                let offset_x = width - target_width;
                let offset_y = height * offset_x / target_width;

                let cut = vec![
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(target_width, height),
                        Point::new(0.0, height),
                    ],
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(offset_x, 0.0),
                        Point::new(offset_x, offset_y),
                    ],
                    vec![
                        Point::new(offset_x, 0.0),
                        Point::new(width, 0.0),
                        Point::new(width, height),
                        Point::new(target_width, height),
                        Point::new(offset_x, offset_y),
                    ],
                ];
                let placed = vec![
                    vec![
                        Point::new(0.0, offset_y),
                        Point::new(target_width, target_height),
                        Point::new(0.0, target_height),
                    ],
                    vec![
                        Point::new(target_width - offset_x, height),
                        Point::new(target_width, height),
                        Point::new(target_width, target_height),
                    ],
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(target_width, 0.0),
                        Point::new(target_width, height),
                        Point::new(target_width - offset_x, height),
                        Point::new(0.0, offset_y),
                    ],
                ];

                return self
                    .manager
                    .add_scissors_and_tape(rectangle.id, &cut, &placed, &target);
            } else {
                let offset_y = height - target_height;
                let offset_x = width * offset_y / target_height;

                let cut = vec![
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(width, 0.0),
                        Point::new(width, target_height),
                    ],
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(offset_x, offset_y),
                        Point::new(0.0, offset_y),
                    ],
                    vec![
                        Point::new(0.0, offset_y),
                        Point::new(offset_x, offset_y),
                        Point::new(width, target_height),
                        Point::new(width, height),
                        Point::new(0.0, height),
                    ],
                ];
                let placed = vec![
                    vec![
                        Point::new(offset_x, 0.0),
                        Point::new(target_width, 0.0),
                        Point::new(target_width, target_height),
                    ],
                    vec![
                        Point::new(target_width - offset_x, target_height - offset_y),
                        Point::new(target_width, target_height),
                        Point::new(target_width - offset_x, target_height),
                    ],
                    vec![
                        Point::new(0.0, 0.0),
                        Point::new(offset_x, 0.0),
                        Point::new(width, target_height - offset_y),
                        Point::new(width, target_height),
                        Point::new(0.0, target_height),
                    ],
                ];

                return self
                    .manager
                    .add_scissors_and_tape(rectangle.id, &cut, &placed, &target);
            }
        }
    }

    // triangle -> canonical triangle -> rectangle -> resize to width x side
    fn triangle_to_rectangle_strip(&mut self, triangle: Shape, width: f64, side: f64) -> Shape {
        // triangle -> canonical triangle (tape)
        let (placed, canonical) = Self::mapping_canonical_triangle(&triangle.polygon);
        let canonical_shape = self
            .manager
            .add_tape(&vec![triangle.id], &vec![placed], &canonical);

        // canonical triangle -> rectangle (scissors + tape)
        let (cut, placed_pieces, rectangle) =
            Self::canonical_triangle_to_rectangle_pieces(&canonical_shape.polygon);
        let rectangle_shape = self.manager.add_scissors_and_tape(
            canonical_shape.id,
            &cut,
            &placed_pieces,
            &rectangle,
        );

        self.resize_rectangle(rectangle_shape, width, side)
    }

    fn tape_rectangles_strip_into_square(&mut self, rectangles: &Vec<Shape>, side: f64) -> Shape {
        let id = rectangles.iter().map(|r| r.id).collect::<Vec<_>>();
        let mut placed = Vec::with_capacity(rectangles.len());
        let mut offset_x = 0.0;

        for rectangle in rectangles {
            let mut moved = rectangle.polygon.clone();

            for point in moved.iter_mut() {
                point.x += offset_x;
            }

            placed.push(moved);

            let width = rectangle.polygon[2].x;
            offset_x += width;
        }

        let square = Self::aabb_rectangle(side, side, 0.0, 0.0);
        self.manager.add_tape(&id, &placed, &square)
    }

    fn cut_square_into_rectangles(
        &mut self,
        square: Shape,
        widths: &Vec<f64>,
        side: f64,
    ) -> Vec<Shape> {
        let mut pieces = Vec::with_capacity(widths.len());
        let mut offset_x = 0.0;

        for &width in widths {
            pieces.push(Self::aabb_rectangle(width, side, offset_x, 0.0));
            offset_x += width;
        }

        self.manager.add_scissors(square.id, &pieces)
    }

    fn rectangle_to_canonical_triangle(&mut self, rectangle: Shape, triangle: &Polygon) -> Shape {
        let (_, canonical) = Self::mapping_canonical_triangle(triangle);

        let width = (rectangle.polygon[1].x - rectangle.polygon[0].x).abs();
        let height = (rectangle.polygon[3].y - rectangle.polygon[0].y).abs();
        let rect = Self::aabb_rectangle(width, height, 0.0, 0.0);
        let normalized = self
            .manager
            .add_tape(&vec![rectangle.id], &vec![rect.clone()], &rect);

        let base_x = canonical[1].x;
        let height_half = canonical[2].y / 2.0;
        let resized = self.resize_rectangle(normalized, base_x, height_half);

        let (cut, placed, triangle_canonical) =
            Self::rectangle_to_canonical_triangle_pieces(&canonical);
        self.manager
            .add_scissors_and_tape(resized.id, &cut, &placed, &triangle_canonical)
    }

    fn tape_triangles_into_target_polygon(
        &mut self,
        triangles: &Vec<Shape>,
        triangles_target: &mut Vec<Polygon>,
    ) -> Shape {
        for i in 0..triangles.len() {
            let edges_triangle = side_lengths(&triangles[i].polygon);

            for _ in 0..3 {
                let edges = side_lengths(&mut triangles_target[i]);

                if approx::eq_vec(&edges_triangle, &edges) {
                    break;
                }

                triangles_target[i].rotate_left(1);
            }
        }

        let ids = triangles.iter().map(|t| t.id).collect::<Vec<_>>();
        self.manager.add_tape(&ids, triangles_target, &self.target)
    }
}

impl WBGSolver {
    fn mapping_canonical_triangle(triangle: &Polygon) -> (Polygon, Polygon) {
        let edges = side_lengths(triangle);
        let mut edges_max = edges.clone();
        let mut placed_rotation = 0;

        let compare = |a: &Vec<f64>, b: &Vec<f64>| -> bool {
            for i in 0..a.len().min(b.len()) {
                if a[i] - b[i] > EPS {
                    return true;
                } else if a[i] - b[i] < -EPS {
                    return false;
                }
            }

            a.len() > b.len()
        };

        let rotation1 = vec![edges[1], edges[2], edges[0]];
        let rotation2 = vec![edges[2], edges[0], edges[1]];

        if compare(&rotation1, &edges_max) {
            edges_max = rotation1;
            placed_rotation = 2;
        }

        if compare(&rotation2, &edges_max) {
            edges_max = rotation2;
            placed_rotation = 1;
        }

        let (x, y, z) = (edges_max[0], edges_max[1], edges_max[2]);
        let center_x = (x * x + z * z - y * y) / (2.0 * x);
        let center_y = (z * z - center_x * center_x).sqrt();
        let canonical = vec![
            Point::new(0.0, 0.0),
            Point::new(x, 0.0),
            Point::new(center_x, center_y),
        ];
        let placed = vec![
            canonical[placed_rotation % 3],
            canonical[(placed_rotation + 1) % 3],
            canonical[(placed_rotation + 2) % 3],
        ];

        (placed, canonical)
    }

    fn aabb_rectangle(width: f64, height: f64, offset_x: f64, offset_y: f64) -> Polygon {
        vec![
            Point::new(offset_x, offset_y),
            Point::new(offset_x + width, offset_y),
            Point::new(offset_x + width, offset_y + height),
            Point::new(offset_x, offset_y + height),
        ]
    }

    fn canonical_triangle_to_rectangle_pieces(
        canonical: &Polygon,
    ) -> (Vec<Polygon>, Vec<Polygon>, Polygon) {
        let (x, center_x, center_y) = (canonical[1].x, canonical[2].x, canonical[2].y);
        let pieces_cut = vec![
            vec![
                Point::new(0.0, 0.0),
                Point::new(x, 0.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x / 2.0, center_y / 2.0),
            ],
            vec![
                Point::new(center_x, center_y / 2.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x, center_y),
            ],
            vec![
                Point::new(center_x / 2.0, center_y / 2.0),
                Point::new(center_x, center_y / 2.0),
                Point::new(center_x, center_y),
            ],
        ];
        let pieces_placed = vec![
            vec![
                Point::new(0.0, 0.0),
                Point::new(x, 0.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x / 2.0, center_y / 2.0),
            ],
            vec![
                Point::new(x, center_y / 2.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(x, 0.0),
            ],
            vec![
                Point::new(center_x / 2.0, center_y / 2.0),
                Point::new(0.0, center_y / 2.0),
                Point::new(0.0, 0.0),
            ],
        ];

        let rectangle = Self::aabb_rectangle(x, center_y / 2.0, 0.0, 0.0);
        (pieces_cut, pieces_placed, rectangle)
    }

    fn rectangle_to_canonical_triangle_pieces(
        canonical: &Polygon,
    ) -> (Vec<Polygon>, Vec<Polygon>, Polygon) {
        let (x, center_x, center_y) = (canonical[1].x, canonical[2].x, canonical[2].y);
        let pieces_cut = vec![
            vec![
                Point::new(0.0, 0.0),
                Point::new(x, 0.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x / 2.0, center_y / 2.0),
            ],
            vec![
                Point::new(x, center_y / 2.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(x, 0.0),
            ],
            vec![
                Point::new(center_x / 2.0, center_y / 2.0),
                Point::new(0.0, center_y / 2.0),
                Point::new(0.0, 0.0),
            ],
        ];
        let pieces_placed = vec![
            vec![
                Point::new(0.0, 0.0),
                Point::new(x, 0.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x / 2.0, center_y / 2.0),
            ],
            vec![
                Point::new(center_x, center_y / 2.0),
                Point::new((x + center_x) / 2.0, center_y / 2.0),
                Point::new(center_x, center_y),
            ],
            vec![
                Point::new(center_x / 2.0, center_y / 2.0),
                Point::new(center_x, center_y / 2.0),
                Point::new(center_x, center_y),
            ],
        ];

        (pieces_cut, pieces_placed, canonical.clone())
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let len_s = scan.token::<usize>();
    let mut polygon_s = Vec::with_capacity(len_s);

    for _ in 0..len_s {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        polygon_s.push(Point::new(x, y));
    }

    let len_t = scan.token::<usize>();
    let mut polygon_t = Vec::with_capacity(len_t);

    for _ in 0..len_t {
        let (x, y) = (scan.token::<f64>(), scan.token::<f64>());
        polygon_t.push(Point::new(x, y));
    }

    let solver = WBGSolver::new(polygon_s, polygon_t);
    let ret = solver.solve();

    for op in ret {
        match op {
            OpType::Scissors(id, pieces) => {
                writeln!(out, "scissors").unwrap();
                writeln!(out, "{id} {}", pieces.len()).unwrap();

                for piece in pieces {
                    write!(out, "{}", piece.len()).unwrap();

                    for point in piece {
                        write!(out, " {:.9} {:.9}", point.x, point.y).unwrap();
                    }

                    writeln!(out).unwrap();
                }
            }
            OpType::Tape(ids, placed, ret) => {
                writeln!(out, "tape").unwrap();

                write!(out, "{}", ids.len()).unwrap();

                for id in ids {
                    write!(out, " {id}").unwrap();
                }

                writeln!(out).unwrap();

                for piece in placed {
                    write!(out, "{}", piece.len()).unwrap();

                    for point in piece {
                        write!(out, " {:.9} {:.9}", point.x, point.y).unwrap();
                    }

                    writeln!(out).unwrap();
                }

                write!(out, "{}", ret.len()).unwrap();

                for point in ret {
                    write!(out, " {:.9} {:.9}", point.x, point.y).unwrap();
                }

                writeln!(out).unwrap();
            }
        }
    }
}
