use std::io;

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn calculate_ccw(p1: (i64, i64), p2: (i64, i64), p3: (i64, i64)) -> i64 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    let res = (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1);
    if res > 0 {
        1
    } else if res < 0 {
        -1
    } else {
        0
    }
}

// Reference: https://jason9319.tistory.com/358
fn is_intersect(x: ((i64, i64), (i64, i64)), y: ((i64, i64), (i64, i64))) -> bool {
    let mut a = x.0;
    let mut b = x.1;
    let mut c = y.0;
    let mut d = y.1;

    let ab = calculate_ccw(a, b, c) * calculate_ccw(a, b, d);
    let cd = calculate_ccw(c, d, a) * calculate_ccw(c, d, b);

    if ab == 0 && cd == 0 {
        if a > b {
            let temp = b;
            b = a;
            a = temp;
        }
        if c > d {
            let temp = d;
            d = c;
            c = temp;
        }

        return c <= b && a <= d;
    }

    ab <= 0 && cd <= 0
}

fn main() {
    let segment1 = input_integers();
    let segment2 = input_integers();

    println!(
        "{}",
        if is_intersect(
            ((segment1[0], segment1[1]), (segment1[2], segment1[3])),
            ((segment2[0], segment2[1]), (segment2[2], segment2[3]))
        ) {
            "1"
        } else {
            "0"
        }
    );
}
