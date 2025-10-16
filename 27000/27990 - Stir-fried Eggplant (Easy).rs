use io::Write;
use std::{io, ops::Rem, str};

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

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;

    if min == 0 && max == 0 {
        return 0;
    } else if min == 0 {
        return max;
    } else if max == 0 {
        return min;
    }

    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;

        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = gcd_extended(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn inverse(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = gcd_extended(a, m);

    if g != 1 {
        None
    } else {
        Some(x.rem_euclid(m))
    }
}

fn calculate(n: i64, t: i64, constraints: &[(i64, i64)]) -> i64 {
    let mut t_scale = 1;

    for &(a, _) in constraints.iter() {
        if a != 0 {
            t_scale = lcm(t_scale, a.abs());
        }
    }

    let u = 4 * n * t_scale;
    let mut crt_residue = 0;
    let mut crt_modulus = 1;

    for &(a, b) in constraints.iter() {
        if a == 0 {
            if b.rem(4 * n) != 0 {
                return 0;
            } else {
                continue;
            }
        }

        let g = gcd(a.abs(), u);
        let c = b.rem_euclid(4 * n) * t_scale;

        if c % g != 0 {
            return 0;
        }

        let a_reduced = a / g;
        let c_reduced = c / g;
        let u_reduced = u / g;

        let a_reduced_mod = a_reduced.rem_euclid(u_reduced);
        let c_reduced_mod = c_reduced.rem_euclid(u_reduced);
        let inv = match inverse(a_reduced_mod, u_reduced) {
            Some(v) => v,
            None => return 0,
        };

        let u_local = (inv * c_reduced_mod) % u_reduced;
        let g = gcd(crt_modulus, u_reduced);
        let diff = (u_local - crt_residue).rem_euclid(u_reduced);

        if diff % g != 0 {
            return 0;
        }

        let crt_reduced = crt_modulus / g;
        let u_reduced2 = u_reduced / g;
        let inv = match inverse(crt_reduced.rem_euclid(u_reduced2), u_reduced2) {
            Some(v) => v,
            None => return 0,
        };

        let step = (diff / g) * inv % u_reduced2;

        crt_residue = (crt_residue + crt_modulus * step).rem_euclid(crt_modulus * u_reduced2);
        crt_modulus = crt_modulus * u_reduced2;
    }

    let residue = crt_residue.rem_euclid(crt_modulus);
    let u_upper = t * t_scale;

    if residue > u_upper {
        0
    } else {
        (u_upper - residue) / crt_modulus + 1
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (n, m, t) = (
        scan.token::<i64>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut ingredients = vec![(0, 0, 0, 0); m];

    for i in 0..m {
        ingredients[i] = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );
    }

    let mut ret = 0;

    for i in 0..m - 1 {
        for j in i + 1..m {
            let (x1, y1, vx1, vy1) = ingredients[i];
            let (x2, y2, vx2, vy2) = ingredients[j];

            let mut sum_total = 0;

            for &dx in [-1, 1].iter() {
                for &dy in [-1, 1].iter() {
                    let constraints = [
                        (vx1 - dx * vx2, dx * (x2 + n) - (x1 + n)),
                        (vy1 - dy * vy2, dy * (y2 + n) - (y1 + n)),
                    ];

                    sum_total += calculate(n, t, &constraints);
                }
            }

            let mut sum_x = 0;

            for &sx in [0, 2 * n].iter() {
                for &dy in [1, -1].iter() {
                    let constraints = [
                        (vx1, sx - (x1 + n)),
                        (vx2, sx - (x2 + n)),
                        (vy1 - dy * vy2, dy * (y2 + n) - (y1 + n)),
                    ];

                    sum_x += calculate(n, t, &constraints);
                }
            }

            let mut sum_y = 0;

            for &sy in [0, 2 * n].iter() {
                for &dx in [1, -1].iter() {
                    let constraints = [
                        (vy1, sy - (y1 + n)),
                        (vy2, sy - (y2 + n)),
                        (vx1 - dx * vx2, dx * (x2 + n) - (x1 + n)),
                    ];

                    sum_y += calculate(n, t, &constraints);
                }
            }

            let mut sum_corner = 0;

            for &sx in [0, 2 * n].iter() {
                for &sy in [0, 2 * n].iter() {
                    let constraints = [
                        (vx1, sx - (x1 + n)),
                        (vx2, sx - (x2 + n)),
                        (vy1, sy - (y1 + n)),
                        (vy2, sy - (y2 + n)),
                    ];

                    sum_corner += calculate(n, t, &constraints);
                }
            }

            ret += sum_total - sum_x - sum_y + sum_corner;
        }
    }

    writeln!(out, "{ret}").unwrap();
}
