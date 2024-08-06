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

fn process_backtrack(
    triangles: &[[i64; 3]; 6],
    visited: &mut [bool; 6],
    triangles_ordered: &mut [i64; 7],
    nums_outside: &mut [i64; 6],
    ret: &mut i64,
    depth: usize,
) {
    if depth == 6 {
        if triangles_ordered[0] != triangles_ordered[6] {
            return;
        }

        let mut sum = 0;

        for i in 0..6 {
            sum += nums_outside[i];
        }

        if sum > *ret {
            *ret = sum;
        }

        return;
    }

    for i in 0..6 {
        if visited[i] {
            continue;
        }

        visited[i] = true;

        for j in 0..3 {
            if triangles[i][j] == triangles_ordered[depth] {
                triangles_ordered[depth + 1] = triangles[i][(j + 1) % 3];
                nums_outside[depth] = triangles[i][(j + 2) % 3];

                process_backtrack(
                    triangles,
                    visited,
                    triangles_ordered,
                    nums_outside,
                    ret,
                    depth + 1,
                );
            }
        }

        visited[i] = false;
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut triangles = [[0; 3]; 6];

        for i in 0..6 {
            for j in 0..3 {
                triangles[i][j] = scan.token::<i64>();
            }
        }

        let mut visited = [false; 6];
        let mut triangles_ordered = [0; 7];
        let mut nums_outside = [0; 6];
        let mut ret = 0;

        visited[0] = true;

        for i in 0..3 {
            triangles_ordered[0] = triangles[0][i];
            triangles_ordered[1] = triangles[0][(i + 1) % 3];
            nums_outside[0] = triangles[0][(i + 2) % 3];

            process_backtrack(
                &triangles,
                &mut visited,
                &mut triangles_ordered,
                &mut nums_outside,
                &mut ret,
                1,
            );
        }

        if ret == 0 {
            writeln!(out, "none").unwrap();
        } else {
            writeln!(out, "{ret}").unwrap();
        }

        let c = scan.token::<String>();

        if c == "$" {
            break;
        }
    }
}
