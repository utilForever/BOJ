use std::{cmp, io, mem, str};

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

fn initialize_grundy(grundy: &mut Vec<Vec<i64>>) {
    for i in 1..grundy.len() {
        grundy[i][0] = i as i64;
    }

    for i in 1..grundy[0].len() {
        grundy[0][i] = i as i64;
    }

    let mut max = cmp::max(grundy.len(), grundy[0].len());

    for i in 1..grundy.len() as i64 {
        for j in 1..grundy[i as usize].len() as i64 {
            let mut visited = vec![false; max + 1];

            let mut k = 1;
            while i - k >= 0 {
                visited[grundy[i as usize - k as usize][j as usize] as usize] = true;
                k += 1;
            }

            k = 1;
            while j - k >= 0 {
                visited[grundy[i as usize][j as usize - k as usize] as usize] = true;
                k += 1;
            }

            k = 1;
            while i - k >= 0 && j - k >= 0 {
                visited[grundy[i as usize - k as usize][j as usize - k as usize] as usize] = true;
                k += 1;
            }

            let mut cnt = 0;
            while cnt < visited.len() && visited[cnt] {
                cnt += 1;
            }

            grundy[i as usize][j as usize] = cnt as i64;
            max = cmp::max(max, grundy[i as usize][j as usize] as usize);
        }
    }
}

fn calculate_grundy(
    cycle: &Vec<i64>,
    grundy: &mut Vec<Vec<i64>>,
    mut row: usize,
    mut column: usize,
) -> i64 {
    if row > column {
        mem::swap(&mut row, &mut column);
    }

    if column < grundy[row].len() {
        return grundy[row][column];
    }

    let steps = (column as i64 - (grundy[row].len() as i64 - 1) + cycle[row] - 1) / cycle[row];
    let column2 = column as i64 - steps * cycle[row as usize];

    grundy[row][column2 as usize] + cycle[row] * steps
}

fn main() {
    let stdin = io::stdin();
    let mut scan = UnsafeScanner::new(stdin.lock());

    let cycle = vec![
        1, 3, 3, 6, 12, 24, 12, 24, 24, 24, 24, 48, 48, 96, 96, 96, 192, 192, 384, 384, 384, 768,
        768, 768, 768,
    ];
    let mut grundy = vec![vec![0; 3800]; 25];

    initialize_grundy(&mut grundy);

    let t = scan.token();

    for _ in 0..t {
        let (_, _, n): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
        let mut ret = 0;

        for _ in 0..n {
            let (row, column): (usize, usize) = (scan.token(), scan.token());
            ret ^= calculate_grundy(&cycle, &mut grundy, row - 1, column - 1);
        }

        if ret != 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
