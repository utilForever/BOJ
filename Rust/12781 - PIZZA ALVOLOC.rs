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
    let a = x.0;
    let b = x.1;
    let c = y.0;
    let d = y.1;

    let ab = calculate_ccw(a, b, c) * calculate_ccw(a, b, d);
    let cd = calculate_ccw(c, d, a) * calculate_ccw(c, d, b);
    
    ab < 0 && cd < 0
}

fn main() {
    let segments = input_integers();

    println!(
        "{}",
        if is_intersect(
            ((segments[0], segments[1]), (segments[2], segments[3])),
            ((segments[4], segments[5]), (segments[6], segments[7]))
        ) {
            "1"
        } else {
            "0"
        }
    );
}
