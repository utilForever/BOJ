use io::Write;
use std::{
    f64::consts::{PI, TAU},
    io, str,
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

const AXIS_ROTATION_DEG: f64 = 23.439281;
const START_DAY_OF_YEAR: [i64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
const MONTHS: [&str; 12] = [
    "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
];

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (latitude_deg, n) = (scan.token::<f64>(), scan.token::<i64>());
    let axis_rotation_rad = AXIS_ROTATION_DEG.to_radians();
    let latitude_rad = latitude_deg.to_radians();
    let latitude_sin = latitude_rad.sin();
    let latitude_cos = latitude_rad.cos();
    let day_solstice = START_DAY_OF_YEAR[5] + 20;

    for _ in 0..n {
        let (d, m, h) = (
            scan.token::<i64>(),
            scan.token::<String>(),
            scan.token::<i64>(),
        );

        let idx = MONTHS.iter().position(|&s| s == m).unwrap();
        let day = START_DAY_OF_YEAR[idx] + d - 1;

        let days_since_solstice = (day - day_solstice) as f64 + (h as f64 - 12.0) / 24.0;
        let solar_longitude = PI * 0.5 + TAU * (days_since_solstice.rem_euclid(365.0)) / 365.0;
        let right_ascension =
            (solar_longitude.sin() * axis_rotation_rad.cos()).atan2(solar_longitude.cos());

        let mut equation_of_time = solar_longitude - right_ascension;
        equation_of_time = equation_of_time.rem_euclid(TAU);

        let hour_angle = (h as f64 - 12.0) * PI / 12.0 + equation_of_time;
        let declination = (axis_rotation_rad.sin() * solar_longitude.sin()).asin();
        let declination_sin = declination.sin();
        let declination_cos = declination.cos();

        let mut altitude_sin =
            latitude_sin * declination_sin + latitude_cos * declination_cos * hour_angle.cos();
        altitude_sin = altitude_sin.clamp(-1.0, 1.0);

        let ret = if altitude_sin > 0.0 {
            altitude_sin.asin().to_degrees()
        } else {
            0.0
        };

        writeln!(out, "{:.12}", ret).unwrap();
    }
}
