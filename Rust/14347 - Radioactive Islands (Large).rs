use std::io;

static MIN_Y_BOUND: f32 = -13.0;
static MAX_Y_BOUND: f32 = 13.0;
static START_X: f32 = -10.0;
static END_X: f32 = 10.0;
static MIN_A: f32 = -10.0;
static MAX_A: f32 = 10.0;
static MIN_C: f32 = -10.0;
static MAX_C: f32 = 10.0;
static X_COORD: f32 = 0.0;
static MIN_SLOPE: f32 = (MIN_C - MAX_A) / (X_COORD - START_X);
static MAX_SLOPE: f32 = (MAX_C - MIN_A) / (X_COORD - START_X);
static H: f32 = 0.01;

fn process_binary_search(a: f32, b: f32, islands: &Vec<f32>, mut left: f32, mut right: f32) -> f32 {
    let mut dose = f32::MAX;

    while (right - left).abs() / 2.0 > f32::EPSILON {
        let mid = (left + right) / 2.0;

        let result = calculate_dose(islands, START_X, a, mid);

        dose = result.0;
        let y = result.1;

        if y < b {
            left = mid;
        } else {
            right = mid;
        }
    }

    dose
}

fn calculate_dose(islands: &Vec<f32>, mut x: f32, mut y: f32, mut yp: f32) -> (f32, f32) {
    let mut dose = 0.0;

    for _ in 0..((END_X - START_X) / H) as i32 {
        if y < MIN_Y_BOUND || y > MAX_Y_BOUND {
            return (f32::MAX, y);
        }

        dose += H * (1.0 + get_determinant(islands, x, y)) * (1.0 + yp * yp).sqrt();

        let k1 = H * yp;
        let l1 = H * calculate_euler_lagrange(islands, x, y, yp);

        x += H;
        y += k1;
        yp += l1;
    }

    (dose, y)
}

fn get_determinant(islands: &Vec<f32>, x: f32, y: f32) -> f32 {
    let mut dose = 0.0;

    for island in islands.iter() {
        let d_square = x * x + (y - island) * (y - island);

        if d_square < f32::EPSILON {
            return f32::MAX;
        }

        dose += 1.0 / d_square;
    }

    dose
}

fn calculate_euler_lagrange(islands: &Vec<f32>, x: f32, y: f32, yp: f32) -> f32 {
    let t = 1.0 + yp * yp;
    let mut s = 1.0;
    let mut syp = 0.0;
    let mut sx = 0.0;

    for island in islands.iter() {
        let d_square = x * x + (y - island) * (y - island);

        s += 1.0 / d_square;
        syp += (y - island) / (d_square * d_square);
        sx += (x + (y - island) * yp) / (d_square * d_square);
    }

    2.0 * t * (sx * yp - syp * t) / s
}

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn input_floating_points() -> Vec<f32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<f32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let t = input_integers()[0];

    for i in 0..t {
        let nums = input_floating_points();

        let n = nums[0] as i32;
        let a = nums[1];
        let b = nums[2];

        let mut islands = vec![0.0; n as usize];

        let nums = input_floating_points();

        for j in 0..n as usize {
            islands[j] = nums[j];
        }

        let mut slopes = vec![MIN_SLOPE, MAX_SLOPE];

        for island in islands.iter() {
            slopes.push((island - a) / (X_COORD - START_X));
        }

        slopes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut min_radiation_dose = f32::MAX;

        for j in 0..slopes.len() - 1 {
            let result = process_binary_search(a, b, &islands, slopes[j], slopes[j + 1]);

            if min_radiation_dose > result {
                min_radiation_dose = result;
            }
        }

        println!("Case #{}: {}", i + 1, min_radiation_dose);
    }
}
