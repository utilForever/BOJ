use io::Write;
use std::{
    io::{self, BufWriter, StdoutLock},
    str,
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

fn pow(x: i64, mut y: i64, rem: i64) -> i64 {
    let mut ret = 1;
    let mut piv = x % rem;

    while y != 0 {
        if y & 1 != 0 {
            ret = ret * piv % rem
        }

        piv = piv * piv % rem;
        y >>= 1;
    }

    ret
}

fn legendre(a: i64, p: i64) -> i32 {
    if a % p == 0 {
        return 0;
    }

    let v = pow(a, (p - 1) / 2, p);

    if v == 1 {
        1
    } else if v == p - 1 {
        -1
    } else {
        0
    }
}

fn find_non_residue(p: i64) -> i64 {
    let mut z = 2;

    while z < p {
        if legendre(z, p) == -1 {
            return z;
        }

        z += 1;
    }

    2
}

fn tonelli_shanks(n: i64, p: i64) -> i64 {
    if p == 2 {
        return n % 2;
    }

    if legendre(n, p) != 1 {
        return -1;
    }

    let mut q = p - 1;
    let mut s = 0;

    while q % 2 == 0 {
        q >>= 1;
        s += 1;
    }

    if s == 1 {
        return pow(n, (p + 1) / 4, p);
    }

    let z = find_non_residue(p);
    let mut m = s;
    let mut c = pow(z, q, p);
    let mut t = pow(n, q, p);
    let mut r = pow(n, (q + 1) / 2, p);

    while t != 1 {
        let mut idx = 1;
        let mut tmp = (t * t) % p;

        while tmp != 1 {
            tmp = (tmp * tmp) % p;
            idx += 1;
        }

        let b = pow(c, 1 << (m - idx - 1), p);

        m = idx;
        c = (b * b) % p;
        t = (t * c) % p;
        r = (r * b) % p;
    }

    r
}

fn print(out: &mut BufWriter<StdoutLock>, x: i64, y: i64) {
    if x == 0 {
        writeln!(out, "0").unwrap();
        return;
    }

    if x == y {
        writeln!(out, "1").unwrap();
        return;
    }

    let g = gcd(x, y);

    writeln!(out, "{}/{}", x / g, y / g).unwrap();
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let t = scan.token::<i64>();

    for _ in 0..t {
        let (p, n, f, d, j, a, b) = (
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
            scan.token::<i64>(),
        );

        if n == 0 {
            writeln!(out, "1").unwrap();
            continue;
        }

        if p == 2 {
            if a % 2 == 0 {
                writeln!(out, "5/6").unwrap();
            } else if b % 2 == 0 {
                writeln!(out, "1").unwrap();
            } else {
                writeln!(out, "2/3").unwrap();
            }

            continue;
        }

        let mut cnt_trivial = 0;
        let mut cnt_possible = 0;
        let mut cnt_total;

        if a == p {
            cnt_trivial += p * p;

            if b == p {
                cnt_trivial += p - 1;
            }

            let g = gcd(n, p - 1);

            if p % 4 == 1 {
                cnt_total = 5 * p * p - 7 * p + 3;

                if b != p {
                    cnt_possible += 4 * (p - 1) * g;

                    if ((p - 1) / g) % 2 == 0 {
                        cnt_possible += 4 * (p - 1) * g;
                    }
                }
            } else {
                cnt_total = 3 * p * p - 3 * p + 1;

                if b != p {
                    cnt_possible += 2 * (p - 1) * g;

                    if ((p - 1) / g) % 2 == 0 {
                        cnt_possible += 2 * (p - 1) * g;
                    }
                }
            }
        } else if a == 1 && b == p {
            cnt_trivial += p * p + p - 1;

            if p % 4 == 1 {
                cnt_total = 5 * p * p - 7 * p + 3;
                cnt_possible += if n % 2 == 0 {
                    4 * (p - 1) * (p - 1)
                } else {
                    2 * (p - 1) * (p - 1)
                };
            } else {
                cnt_total = 3 * p * p - 3 * p + 1;
                cnt_possible += 2 * (p - 1) * (p - 1);
            }
        } else if b == p {
            cnt_trivial += p * p + p - 1;

            if p % 4 == 1 {
                cnt_total = 5 * p * p - 7 * p + 3;

                let x = pow(a, n % (p - 1), p);

                if n % 2 == 0 {
                    if x == 1 || x == p - 1 {
                        cnt_possible += 4 * (p - 1) * (p - 1);
                    }
                } else {
                    if x == 1 || x == p - 1 {
                        cnt_possible += 2 * (p - 1) * (p - 1);
                    }

                    let y = pow(a, (2 * n) % (p - 1), p);

                    if y == p - 1 {
                        cnt_possible += 2 * (p - 1) * (p - 1);
                    }
                }
            } else {
                cnt_total = 3 * p * p - 3 * p + 1;

                let x = pow(a, n % (p - 1), p);

                if x == 1 || x == p - 1 {
                    cnt_possible += 2 * (p - 1) * (p - 1);
                }
            }
        } else {
            cnt_trivial += p * p;
            cnt_total = 3 * p * p - 3 * p + 1;

            let g = gcd(n, p - 1);
            let x = pow(a, n % (p - 1), p);
            let y = pow(p - a, n % (p - 1), p);

            cnt_possible += if x == 1 {
                (g - 1) * (p - 1)
            } else {
                g * (p - 1)
            };

            if pow(p - 1, (p - 1) / g, p) == 1 {
                cnt_possible += if x == p - 1 {
                    (g - 1) * (p - 1)
                } else {
                    g * (p - 1)
                }
            }

            cnt_possible += if y == 1 {
                (g - 1) * (p - 1)
            } else {
                g * (p - 1)
            };

            if pow(p - 1, (p - 1) / g, p) == 1 {
                cnt_possible += if y == p - 1 {
                    (g - 1) * (p - 1)
                } else {
                    g * (p - 1)
                }
            }

            if p % 4 == 1 {
                cnt_total = 5 * p * p - 7 * p + 3;

                let s = tonelli_shanks(p - 1, p);
                let x = pow(a * s % p, n % (p - 1), p);
                let y = pow(p - a * s % p, n % (p - 1), p);

                cnt_possible += if x == 1 {
                    (g - 1) * (p - 1)
                } else {
                    g * (p - 1)
                };

                if pow(p - 1, (p - 1) / g, p) == 1 {
                    cnt_possible += if x == p - 1 {
                        (g - 1) * (p - 1)
                    } else {
                        g * (p - 1)
                    }
                }

                cnt_possible += if y == 1 {
                    (g - 1) * (p - 1)
                } else {
                    g * (p - 1)
                };

                if pow(p - 1, (p - 1) / g, p) == 1 {
                    cnt_possible += if y == p - 1 {
                        (g - 1) * (p - 1)
                    } else {
                        g * (p - 1)
                    }
                }
            }
        }

        if f == 0 && d != 0 {
            print(&mut out, cnt_trivial + cnt_possible, cnt_total);
        } else if f != 0 && j != 0 {
            print(&mut out, cnt_trivial, cnt_total);
        } else {
            print(&mut out, cnt_trivial + cnt_possible / 2, cnt_total);
        }
    }
}
