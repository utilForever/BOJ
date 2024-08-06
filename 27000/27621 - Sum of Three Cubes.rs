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
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let solution: [Option<(i64, i64, i64)>; 50] = [
        Some((0, 0, 0)),
        Some((0, 0, 1)),
        Some((0, 1, 1)),
        Some((1, 1, 1)),
        None,
        None,
        Some((-1, -1, 2)),
        Some((0, -1, 2)),
        Some((0, 0, 2)),
        Some((0, 1, 2)),
        Some((1, 1, 2)),
        Some((-2, -2, 3)),
        Some((7, 10, -11)),
        None,
        None,
        Some((-1, 2, 2)),
        Some((-511, -1609, 1626)),
        Some((1, 2, 2)),
        Some((-1, -2, 3)),
        Some((0, -2, 3)),
        Some((1, -2, 3)),
        Some((-11, -14, 16)),
        None,
        None,
        Some((-2901096694, -15550555555, 15584139827)),
        Some((-1, -1, 3)),
        Some((0, -1, 3)),
        Some((0, 0, 3)),
        Some((0, 1, 3)),
        Some((1, 1, 3)),
        Some((-283059965, -2218888517, 2220422932)),
        None,
        None,
        Some((8866128975287528, -8778405442862239, -2736111468807040)),
        Some((-1, 2, 3)),
        Some((0, 2, 3)),
        Some((1, 2, 3)),
        Some((0, -3, 4)),
        Some((1, -3, 4)),
        Some((117367, 134476, -159380)),
        None,
        None,
        Some((-80538738812075974, 80435758145817515, 12602123297335631)),
        Some((2, 2, 3)),
        Some((-5, -7, 8)),
        Some((2, -3, 4)),
        Some((-2, 3, 3)),
        Some((6, 7, -8)),
        Some((-23, -26, 31)),
        None,
    ];

    let n = scan.token::<usize>();

    writeln!(
        out,
        "{}",
        match solution[n] {
            Some((x, y, z)) => format!("{x} {y} {z}"),
            None => 0.to_string(),
        }
    )
    .unwrap();
}
