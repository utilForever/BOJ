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

// const CIRCLE: f64 = 6.28318530717958647692;
// const TRIANGLE: f64 = 5.19615242270663286060;
// const SQUARE: f64 = 5.65685424949238019521;
// const CIRCLE_TO_TRIANGLE: f64 = 1.65398668626537621301;
// const CIRCLE_TO_SQUARE: f64 = 1.27323954473516268615;
// const TRIANGLE_TO_CIRCLE: f64 = 1.20919957615614523361;
// const TRIANGLE_TO_SQUARE: f64 = 1.28790110171875771566;
// const SQUARE_TO_CIRCLE: f64 = 1.11072073453959156150;
// const SQUARE_TO_TRIANGLE: f64 = 1.61602540378443864677;

// enum Shape {
//     Circle,
//     Triangle,
//     Square,
// }

// fn mex(a: i64, b: i64, c: i64) -> i64 {
//     for i in 0..3 {
//         if a != i && b != i && c != i {
//             return i;
//         }
//     }

//     3
// }

// fn calculate(length: f64, shape: Shape) -> i64 {
//     if length + 1e-12 < 1.0 {
//         return 0;
//     }

//     match shape {
//         Shape::Circle => mex(
//             calculate(length - 1.0, Shape::Circle),
//             calculate(length / CIRCLE_TO_TRIANGLE - 1.0, Shape::Triangle),
//             calculate(length / CIRCLE_TO_SQUARE - 1.0, Shape::Square),
//         ),
//         Shape::Triangle => mex(
//             calculate(length - 1.0, Shape::Triangle),
//             calculate(length / TRIANGLE_TO_CIRCLE - 1.0, Shape::Circle),
//             calculate(length / TRIANGLE_TO_SQUARE - 1.0, Shape::Square),
//         ),
//         Shape::Square => mex(
//             calculate(length - 1.0, Shape::Square),
//             calculate(length / SQUARE_TO_CIRCLE - 1.0, Shape::Circle),
//             calculate(length / SQUARE_TO_TRIANGLE - 1.0, Shape::Triangle),
//         ),
//     }
// }

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let grundy = [
        0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 2, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1,
        1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 3, 3, 3, 1, 0, 0, 0, 1, 1, 1, 2, 2, 2, 1, 1, 1, 1, 3, 3,
        2, 0, 0, 0, 0, 0, 2, 3, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 0, 3, 3, 2, 1, 1, 2, 2, 2, 2, 0, 0,
        0, 0, 3, 1, 1, 1, 1, 1, 1, 2, 0, 0, 0, 0, 0, 2, 1, 1, 1, 1, 1, 1, 2, 3, 0, 0, 0, 2, 1, 1,
        1, 1, 3, 2, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 2, 1, 2, 1, 0, 0, 0, 0, 0,
        2, 2, 1, 1, 1, 3, 3, 2, 0, 0, 3, 2, 1, 1, 1, 1, 1, 1, 0, 0, 2, 2, 2, 0, 0, 0, 0, 1, 3, 1,
        1, 1, 1, 2, 3, 3, 0, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 0, 0, 0,
        0, 0, 1, 1, 1, 1, 0, 3, 3, 0, 0, 2, 3, 0, 2, 2, 2, 0, 1, 2, 1, 0, 0, 2, 3, 3, 2, 3, 1, 0,
        0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 2, 2,
        2, 2, 0, 0, 0, 0, 0, 0, 2, 1, 2, 2, 2, 1, 3, 0, 1, 1, 1, 0, 0, 0, 0, 0, 3, 1, 1, 3, 1, 1,
        1,
    ];
    // let mut grundy = vec![0; 301];

    // for i in 6..=300 {
    //     grundy[i as usize] = mex(
    //         calculate(i as f64 / CIRCLE - 1.0, Shape::Circle),
    //         calculate(i as f64 / TRIANGLE - 1.0, Shape::Triangle),
    //         calculate(i as f64 / SQUARE - 1.0, Shape::Square),
    //     );
    // }

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<usize>();
        let mut ret = 0;

        for _ in 0..n {
            let length = scan.token::<usize>();
            ret ^= grundy[length];
        }

        writeln!(
            out,
            "{}",
            if ret == 0 {
                "Curse will be forever!"
            } else {
                "No More Curse!"
            }
        )
        .unwrap();
    }
}
