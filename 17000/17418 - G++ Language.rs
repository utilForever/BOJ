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

fn input(out: &mut BufWriter<StdoutLock>, x: usize) {
    writeln!(out, "INPUT {x}").unwrap();
}

fn not(out: &mut BufWriter<StdoutLock>, x: usize, y: usize) {
    writeln!(out, "NOT {x} {y}").unwrap();
}

fn bnot(out: &mut BufWriter<StdoutLock>, x: usize, y: usize) {
    writeln!(out, "BNOT {x} {y}").unwrap();
}

fn and(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    writeln!(out, "AND 2 {x} {y} {z}").unwrap();
}

fn or(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    writeln!(out, "OR 2 {x} {y} {z}").unwrap();
}

fn xor(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    writeln!(out, "XOR 2 {x} {y} {z}").unwrap();
}

fn lshift(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    writeln!(out, "LSHIFT {x} {y} {z}").unwrap();
}

fn rshift(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    writeln!(out, "RSHIFT {x} {y} {z}").unwrap();
}

fn mov(out: &mut BufWriter<StdoutLock>, x: usize, y: usize) {
    bnot(out, x, 99);
    bnot(out, 99, y);
}

fn zero(out: &mut BufWriter<StdoutLock>, x: usize) {
    and(out, x, 200, x);
}

fn neg(out: &mut BufWriter<StdoutLock>, x: usize, y: usize) {
    bnot(out, x, 99);
    add(out, 99, 201, y);
}

fn add(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    mov(out, x, 101);
    mov(out, y, 102);

    for i in 0..12 {
        xor(out, 101, 102, 104 + i);
        and(out, 101, 102, 103);
        lshift(out, 103, 201, 103);
        mov(out, 104 + i, 101);
        mov(out, 103, 102);
    }

    mov(out, 115, z);
    zero(out, 103);
}

fn sub(out: &mut BufWriter<StdoutLock>, x: usize, y: usize, z: usize) {
    mov(out, y, 100);
    neg(out, 100, 100);
    add(out, x, 100, z);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (h, w) = (scan.token::<usize>(), scan.token::<usize>());

    // Input 'H W A_0 A_1 … A_{H×W-1} a b c d'
    {
        // H, W: Don't care (0)
        input(&mut out, 0);
        input(&mut out, 0);
        zero(&mut out, 0);

        // A_0 … A_{H×W-1}: 5 ~ H×W+4
        // NOTE: H×W <= 16 => (H×W+4) <= 20
        for i in 0..(h * w) {
            input(&mut out, i + 5);
        }

        // a, b, c, d: 1 ~ 4
        input(&mut out, 1);
        input(&mut out, 2);
        input(&mut out, 3);
        input(&mut out, 4);
    }

    {
        // 0: 200
        // 1: 201
        not(&mut out, 201, 201);

        // 2 ~ 16: 202 ~ 216
        for i in 2..=16 {
            add(&mut out, 200 + i - 1, 201, 200 + i);
        }
    }

    // Flag to add: 301 ~ 316
    // Coord (y, x): 401, 402
    // a, c: y coord
    // b, d; x coord
    {
        for j in 0..h {
            for i in 0..w {
                mov(&mut out, 200, 501);

                // b <= i
                mov(&mut out, 200 + i, 402);
                sub(&mut out, 402, 2, 402);

                // Check
                rshift(&mut out, 402, 211, 402);
                and(&mut out, 402, 201, 402);
                or(&mut out, 501, 402, 501);

                // i <= d
                mov(&mut out, 200 + i, 402);
                sub(&mut out, 4, 402, 402);

                // Check
                rshift(&mut out, 402, 211, 402);
                and(&mut out, 402, 201, 402);
                or(&mut out, 501, 402, 501);

                // a <= j
                mov(&mut out, 200 + j, 401);
                sub(&mut out, 401, 1, 401);

                // Check
                rshift(&mut out, 401, 211, 401);
                and(&mut out, 401, 201, 401);
                or(&mut out, 501, 401, 501);

                // j <= c
                mov(&mut out, 200 + j, 401);
                sub(&mut out, 3, 401, 401);

                // Check
                rshift(&mut out, 401, 211, 401);
                and(&mut out, 401, 201, 401);
                or(&mut out, 501, 401, 501);

                zero(&mut out, 502);
                bnot(&mut out, 502, 502);
                add(&mut out, 502, 501, 502);
                mov(&mut out, 502, 301 + j * w + i);
            }
        }
    }

    {
        // Calculate result
        for i in 0..(h * w) {
            and(&mut out, 5 + i, 301 + i, 501);
            add(&mut out, 0, 501, 0);
        }

        // Reset
        for i in 1..=20 {
            zero(&mut out, i);
        }
        for i in 99..=115 {
            zero(&mut out, i);
        }
        for i in 201..=216 {
            zero(&mut out, i);
        }
        for i in 301..=316 {
            zero(&mut out, i);
        }
        zero(&mut out, 401);
        zero(&mut out, 402);
        zero(&mut out, 501);
        zero(&mut out, 502);
    }
}
