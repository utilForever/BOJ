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

fn process_stars(map: &mut Vec<Vec<char>>, depth: i64, row: i64, col: i64) {
    if depth == 1 {
        map[row as usize][col as usize] = '*';
        return;
    }

    let width = 2_i64.pow(depth as u32 + 1) - 3;
    let height = 2_i64.pow(depth as u32) - 1;

    if depth % 2 == 1 {
        for i in 0..width {
            map[(row + height - 1) as usize][(col + i) as usize] = '*';
        }

        for i in 0..height - 1 {
            map[(row + i) as usize][(col + width / 2 - i) as usize] = '*';
            map[(row + i) as usize][(col + width / 2 + i) as usize] = '*';
        }

        process_stars(
            map,
            depth - 1,
            row + height / 2,
            col + 2_i64.pow(depth as u32 - 1),
        );
    } else {
        for i in 0..width {
            map[row as usize][(col + i) as usize] = '*';
        }

        for i in 1..height {
            map[(row + i) as usize][(col + i) as usize] = '*';
            map[(row + i) as usize][(col + width - i - 1) as usize] = '*';
        }

        process_stars(
            map,
            depth - 1,
            row + 1,
            col + 2_i64.pow(depth as u32 - 1),
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let width = 2_i64.pow(n as u32 + 1) - 3;
    let height = 2_i64.pow(n as u32) - 1;
    let mut map = vec![vec![' '; width as usize]; height as usize];

    process_stars(&mut map, n as i64, 0, 0);

    for i in 0..height {
        let idx = if n % 2 == 1 {
            width - height + i + 1
        } else {
            width - i
        };

        for j in 0..idx {
            write!(out, "{}", map[i as usize][j as usize]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
