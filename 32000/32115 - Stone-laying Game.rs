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
    let mut positions = vec![0; n];
    let mut scores = vec![0; n];

    for i in 0..n {
        positions[i] = scan.token::<i64>();
    }

    for i in 0..n {
        scores[i] = scan.token::<i64>();
    }

    if positions.iter().all(|&x| x == 0) {
        writeln!(out, "0 0").unwrap();
        return;
    }

    if positions.iter().any(|&x| x == 1) && !positions.iter().any(|&x| x == 2) {
        writeln!(out, "{} 0", scores.iter().sum::<i64>()).unwrap();
        return;
    }

    if positions.iter().any(|&x| x == 2) && !positions.iter().any(|&x| x == 1) {
        writeln!(out, "0 {}", scores.iter().sum::<i64>()).unwrap();
        return;
    }

    let mut mid = Vec::new();
    let mut score_cheolsu = 0;
    let mut score_yonghee = 0;

    for i in 0..n {
        if positions[i] == 1 {
            score_cheolsu += scores[i];
        } else if positions[i] == 2 {
            score_yonghee += scores[i];
        }
    }

    for i in 0..n {
        if positions[i] == 1 || positions[i] == 2 {
            let partial_positions = positions[..=i].to_vec();
            let partial_scores = scores[..=i].to_vec();

            positions.extend(partial_positions);
            scores.extend(partial_scores);

            break;
        }
    }

    let mut pos_a = -1;
    let mut pos_b = -1;

    for i in 0..positions.len() {
        if positions[i] == 0 {
            continue;
        }

        if pos_a == -1 {
            pos_a = i as i64;
        } else {
            pos_b = i as i64;
        }

        if pos_a != -1 && pos_b != -1 {
            let stone_a = positions[pos_a as usize];
            let stone_b = positions[pos_b as usize];
            let len = (pos_b - pos_a - 1) as usize;
            let idx = pos_a as usize + 1;

            if stone_a == 1 && stone_b == 1 {
                for i in 0..len {
                    score_cheolsu += scores[idx + i];
                }
            } else if stone_a == 2 && stone_b == 2 {
                for i in 0..len {
                    score_yonghee += scores[idx + i];
                }
            } else if stone_a == 1 && stone_b == 2 {
                if len % 2 == 0 {
                    for i in 0..len {
                        if i < len / 2 {
                            score_cheolsu += scores[idx + i];
                        } else {
                            score_yonghee += scores[idx + i];
                        }
                    }
                } else {
                    for i in 0..len {
                        if i == len / 2 {
                            mid.push(scores[idx + i]);
                        } else if i < len / 2 {
                            score_cheolsu += scores[idx + i];
                        } else {
                            score_yonghee += scores[idx + i];
                        }
                    }
                }
            } else if stone_a == 2 && stone_b == 1 {
                if len % 2 == 0 {
                    for i in 0..len {
                        if i < len / 2 {
                            score_yonghee += scores[idx + i];
                        } else {
                            score_cheolsu += scores[idx + i];
                        }
                    }
                } else {
                    for i in 0..len {
                        if i == len / 2 {
                            mid.push(scores[idx + i]);
                        } else if i < len / 2 {
                            score_yonghee += scores[idx + i];
                        } else {
                            score_cheolsu += scores[idx + i];
                        }
                    }
                }
            }

            pos_a = pos_b;
            pos_b = -1;
        }
    }

    mid.sort_by(|a, b| b.cmp(a));

    for i in 0..mid.len() {
        if i % 2 == 0 {
            score_cheolsu += mid[i];
        } else {
            score_yonghee += mid[i];
        }
    }

    writeln!(out, "{score_cheolsu} {score_yonghee}").unwrap();
}
