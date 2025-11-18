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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut bingo = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            bingo[i][j] = scan.token::<i64>();
        }
    }

    let mut sum_row = vec![0; n];
    let mut sum_col = vec![0; n];
    let mut sum_diag1 = 0;
    let mut sum_diag2 = 0;

    for i in 0..n {
        for j in 0..n {
            sum_row[i] += bingo[i][j];
            sum_col[j] += bingo[i][j];

            if i == j {
                sum_diag1 += bingo[i][j];
            }

            if i + j == n - 1 {
                sum_diag2 += bingo[i][j];
            }
        }
    }

    let cnt_total = 2 * n + 2;
    let mut row_finished = vec![false; n];
    let mut col_finished = vec![false; n];
    let mut diag1_finished = false;
    let mut diag2_finished = false;

    let mut solved = vec![vec![0; n]; n];
    let mut time = 0;
    let mut cnt_finished = 0;
    let mut ret = vec![0; cnt_total];

    while cnt_finished < cnt_total {
        let mut best_sum = i64::MAX;
        let mut best_type = 255;
        let mut best_idx = usize::MAX;

        for i in 0..n {
            if row_finished[i] {
                continue;
            }

            if best_sum > sum_row[i]
                || (best_sum == sum_row[i] && (best_type > 0 || (best_type == 0 && best_idx > i)))
            {
                best_sum = sum_row[i];
                best_type = 0;
                best_idx = i;
            }
        }

        for j in 0..n {
            if col_finished[j] {
                continue;
            }

            if best_sum > sum_col[j]
                || (best_sum == sum_col[j] && (best_type > 1 || (best_type == 1 && best_idx > j)))
            {
                best_sum = sum_col[j];
                best_type = 1;
                best_idx = j;
            }
        }

        if !diag1_finished {
            if best_sum > sum_diag1 || (best_sum == sum_diag1 && best_type > 2) {
                best_sum = sum_diag1;
                best_type = 2;
                best_idx = 0;
            }
        }

        if !diag2_finished {
            if best_sum > sum_diag2 || (best_sum == sum_diag2 && best_type > 3) {
                best_sum = sum_diag2;
                best_type = 3;
                best_idx = 0;
            }
        }

        time += best_sum;

        let mut cnt = 0;

        match best_type {
            0 => {
                let i = best_idx;

                if !row_finished[i] {
                    row_finished[i] = true;
                    cnt += 1;
                }

                for j in 0..n {
                    if solved[i][j] == 0 {
                        solved[i][j] = 1;

                        if !col_finished[j] {
                            sum_col[j] -= bingo[i][j];

                            if sum_col[j] == 0 {
                                col_finished[j] = true;
                                cnt += 1;
                            }
                        }

                        if i == j && !diag1_finished {
                            sum_diag1 -= bingo[i][j];

                            if sum_diag1 == 0 {
                                diag1_finished = true;
                                cnt += 1;
                            }
                        }

                        if i + j == n - 1 && !diag2_finished {
                            sum_diag2 -= bingo[i][j];

                            if sum_diag2 == 0 {
                                diag2_finished = true;
                                cnt += 1;
                            }
                        }
                    }
                }

                sum_row[i] = 0;
            }
            1 => {
                let j = best_idx;

                if !col_finished[j] {
                    col_finished[j] = true;
                    cnt += 1;
                }

                for i in 0..n {
                    if solved[i][j] == 0 {
                        solved[i][j] = 1;

                        if !row_finished[i] {
                            sum_row[i] -= bingo[i][j];

                            if sum_row[i] == 0 {
                                row_finished[i] = true;
                                cnt += 1;
                            }
                        }

                        if i == j && !diag1_finished {
                            sum_diag1 -= bingo[i][j];

                            if sum_diag1 == 0 {
                                diag1_finished = true;
                                cnt += 1;
                            }
                        }

                        if i + j == n - 1 && !diag2_finished {
                            sum_diag2 -= bingo[i][j];

                            if sum_diag2 == 0 {
                                diag2_finished = true;
                                cnt += 1;
                            }
                        }
                    }
                }

                sum_col[j] = 0;
            }
            2 => {
                if !diag1_finished {
                    diag1_finished = true;
                    cnt += 1;
                }

                for i in 0..n {
                    let j = i;

                    if solved[i][j] == 0 {
                        solved[i][j] = 1;

                        if !row_finished[i] {
                            sum_row[i] -= bingo[i][j];

                            if sum_row[i] == 0 {
                                row_finished[i] = true;
                                cnt += 1;
                            }
                        }

                        if !col_finished[j] {
                            sum_col[j] -= bingo[i][j];

                            if sum_col[j] == 0 {
                                col_finished[j] = true;
                                cnt += 1;
                            }
                        }
                        if i + j == n - 1 && !diag2_finished {
                            sum_diag2 -= bingo[i][j];

                            if sum_diag2 == 0 {
                                diag2_finished = true;
                                cnt += 1;
                            }
                        }
                    }
                }

                sum_diag1 = 0;
            }
            3 => {
                if !diag2_finished {
                    diag2_finished = true;
                    cnt += 1;
                }

                for i in 0..n {
                    let j = n - 1 - i;

                    if solved[i][j] == 0 {
                        solved[i][j] = 1;

                        if !row_finished[i] {
                            sum_row[i] -= bingo[i][j];

                            if sum_row[i] == 0 {
                                row_finished[i] = true;
                                cnt += 1;
                            }
                        }

                        if !col_finished[j] {
                            sum_col[j] -= bingo[i][j];

                            if sum_col[j] == 0 {
                                col_finished[j] = true;
                                cnt += 1;
                            }
                        }

                        if i == j && !diag1_finished {
                            sum_diag1 -= bingo[i][j];

                            if sum_diag1 == 0 {
                                diag1_finished = true;
                                cnt += 1;
                            }
                        }
                    }
                }

                sum_diag2 = 0;
            }
            _ => unreachable!(),
        }

        for _ in 0..cnt {
            ret[cnt_finished] = time;
            cnt_finished += 1;
        }
    }

    for val in ret {
        writeln!(out, "{val}").unwrap();
    }
}
