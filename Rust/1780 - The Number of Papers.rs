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

fn divide_paper(
    paper: &Vec<Vec<i32>>,
    y_start: usize,
    y_end: usize,
    x_start: usize,
    x_end: usize,
    num_minus_paper: &mut usize,
    num_zero_paper: &mut usize,
    num_plus_paper: &mut usize,
) {
    let start_color = paper[y_start][x_start];
    let mut is_all_same_color = true;

    for i in y_start..y_end {
        for j in x_start..x_end {
            if paper[i][j] != start_color {
                is_all_same_color = false;
                break;
            }
        }
    }

    if is_all_same_color {
        if start_color == -1 {
            *num_minus_paper += 1;
        } else if start_color == 0 {
            *num_zero_paper += 1;
        } else {
            *num_plus_paper += 1;
        }
    } else {
        let y_part1 = y_start + (y_end - y_start) / 3;
        let y_part2 = y_start + 2 * (y_end - y_start) / 3;
        let x_part1 = x_start + (x_end - x_start) / 3;
        let x_part2 = x_start + 2 * (x_end - x_start) / 3;

        divide_paper(
            paper,
            y_start,
            y_part1,
            x_start,
            x_part1,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_start,
            y_part1,
            x_part1,
            x_part2,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_start,
            y_part1,
            x_part2,
            x_end,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part1,
            y_part2,
            x_start,
            x_part1,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part1,
            y_part2,
            x_part1,
            x_part2,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part1,
            y_part2,
            x_part2,
            x_end,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part2,
            y_end,
            x_start,
            x_part1,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part2,
            y_end,
            x_part1,
            x_part2,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
        divide_paper(
            paper,
            y_part2,
            y_end,
            x_part2,
            x_end,
            num_minus_paper,
            num_zero_paper,
            num_plus_paper,
        );
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut paper = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            paper[i][j] = scan.token::<i32>();
        }
    }

    let mut num_minus_paper = 0;
    let mut num_zero_paper = 0;
    let mut num_plus_paper = 0;

    divide_paper(
        &paper,
        0,
        n,
        0,
        n,
        &mut num_minus_paper,
        &mut num_zero_paper,
        &mut num_plus_paper,
    );

    writeln!(out, "{}", num_minus_paper).unwrap();
    writeln!(out, "{}", num_zero_paper).unwrap();
    writeln!(out, "{}", num_plus_paper).unwrap();
}
