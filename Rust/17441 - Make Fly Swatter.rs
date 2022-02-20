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

    (x2 - x1) * (y3 - y1) - (x3 - x1) * (y2 - y1)
}

fn main() {
    let n = input_integers()[0] as usize;
    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        let values = input_integers();

        x[i] = values[0];
        y[i] = values[1];
    }

    let mut area: f64 = 0.0;
    for i in 1..(n - 1) {
        area += calculate_ccw((x[0], y[0]), (x[i], y[i]), (x[i + 1], y[i + 1])) as f64 / 2.0;
    }

    let mut integral_x_square = 0.0;
    let mut integral_y_square = 0.0;
    let mut integral_x = 0.0;
    let mut integral_y = 0.0;

    for i in 0..n {
        integral_x_square += ((x[(i + 1) % n].pow(3)
            + x[(i + 1) % n].pow(2) * x[i]
            + x[(i + 1) % n] * x[i].pow(2)
            + x[i].pow(3))
            * (y[(i + 1) % n] - y[i])) as f64
            / 12.0;
        integral_y_square += ((y[(i + 1) % n].pow(3)
            + y[(i + 1) % n].pow(2) * y[i]
            + y[(i + 1) % n] * y[i].pow(2)
            + y[i].pow(3))
            * (x[(i + 1) % n] - x[i])) as f64
            / -12.0;
        integral_x += ((x[(i + 1) % n].pow(2) + x[(i + 1) % n] * x[i] + x[i].pow(2))
            * (y[(i + 1) % n] - y[i])) as f64
            / 6.0;
        integral_y += ((y[(i + 1) % n].pow(2) + y[(i + 1) % n] * y[i] + y[i].pow(2))
            * (x[(i + 1) % n] - x[i])) as f64
            / -6.0;
    }

    let ans = (area * integral_x_square + area * integral_y_square
        - integral_x.powf(2.0)
        - integral_y.powf(2.0)) as f64
        * 2.0
        / area.powf(2.0);
    println!("{}", ans);
}
