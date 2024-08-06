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
}

fn process_move(a: &mut i64, b: &mut i64, ret: &mut Vec<i64>, pos: i64) {
    if *a < pos {
        *a += 1;
    } else if *a > pos {
        *a -= 1;
    }

    if *b < pos {
        *b -= 1;
    } else if *b > pos {
        *b += 1;
    }

    ret.push(pos);
}

fn flip(a: &mut i64, b: &mut i64, n: i64, ret: &mut Vec<i64>) {
    let diff = (*a - *b).abs();

    if a < b {
        for _ in 0..diff {
            process_move(a, b, ret, n);
        }
    } else {
        for _ in 0..diff {
            process_move(a, b, ret, 0);
        }
    }
}

fn print(out: &mut BufWriter<StdoutLock>, can_meet: bool, ret: &Vec<i64>) {
    if can_meet {
        writeln!(out, "YES").unwrap();

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<i64>();
    let (mut a, mut b, mut c, mut d) = (
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
        scan.token::<i64>(),
    );
    let mut ret = Vec::new();

    if a == c && b == d {
        print(&mut out, true, &ret);
        return;
    }

    if a == d && b == c {
        flip(&mut a, &mut b, n, &mut ret);
        print(&mut out, true, &ret);
        return;
    }

    if a == b && (a == 1 || a == n - 1) {
        print(&mut out, false, &ret);
        return;
    }

    let mut op = 0;

    if c == d {
        c -= 1;
        d += 1;
        op = 1;
    } else if c + 1 == d {
        c -= 1;
        d += 1;
        op = 1;
    } else if d + 1 == c {
        c += 1;
        d -= 1;
        op = 2;
    }

    if c <= 0 || d <= 0 || c >= n || d >= n {
        print(&mut out, false, &ret);
        return;
    }

    if a < b {
        while b - a > 2 {
            process_move(&mut a, &mut b, &mut ret, n);
        }
    } else {
        while a >= b {
            process_move(&mut a, &mut b, &mut ret, 0);
        }
    }

    if a + 1 == b {
        if b != n - 1 {
            let val_a = a;
            process_move(&mut a, &mut b, &mut ret, val_a);
        } else {
            flip(&mut a, &mut b, n, &mut ret);
            process_move(&mut a, &mut b, &mut ret, n - 1);
            flip(&mut a, &mut b, n, &mut ret);
        }
    }

    if c < d {
        if a <= c {
            while a < c {
                let val_a = a + 1;
                process_move(&mut a, &mut b, &mut ret, val_a);
            }
        } else {
            flip(&mut a, &mut b, n, &mut ret);

            while b > c {
                let val_b = b + 1;
                process_move(&mut a, &mut b, &mut ret, val_b);
            }

            flip(&mut a, &mut b, n, &mut ret);
        }

        while b < d {
            let val_a = a;
            process_move(&mut a, &mut b, &mut ret, val_a);
        }
    } else {
        if b <= c {
            while b < c {
                let val_a = a + 1;
                process_move(&mut a, &mut b, &mut ret, val_a);
            }

            flip(&mut a, &mut b, n, &mut ret);
        } else {
            flip(&mut a, &mut b, n, &mut ret);

            while a > c {
                let val_b = b + 1;
                process_move(&mut a, &mut b, &mut ret, val_b);
            }
        }

        while b > d {
            let val_a = a;
            process_move(&mut a, &mut b, &mut ret, val_a);
        }
    }

    if op == 1 {
        process_move(&mut a, &mut b, &mut ret, n);
    } else if op == 2 {
        process_move(&mut a, &mut b, &mut ret, 0);
    }

    print(&mut out, true, &ret);
}
