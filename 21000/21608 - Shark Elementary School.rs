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

#[derive(Clone, Default)]
struct Student {
    num: i64,
    favorites: [i64; 4],
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let n = scan.token::<usize>();
    let mut students = vec![Student::default(); n * n];
    let mut classroom = vec![vec![0; n]; n];

    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];

    for i in 0..n * n {
        students[i].num = scan.token::<i64>();

        for j in 0..4 {
            students[i].favorites[j] = scan.token::<i64>();
        }
    }

    let batch_student = |classroom: &mut Vec<Vec<i64>>, student: &Student| {
        let mut empty_seats = Vec::new();

        for i in 0..n {
            for j in 0..n {
                if classroom[i][j] != 0 {
                    continue;
                }

                let mut cnt_slots = 0;
                let mut cnt_favorites = 0;

                for k in 0..4 {
                    let nx = i as i64 + dx[k];
                    let ny = j as i64 + dy[k];

                    if nx < 0 || nx >= n as i64 || ny < 0 || ny >= n as i64 {
                        continue;
                    }

                    let student_next = classroom[nx as usize][ny as usize];

                    if student_next == 0 {
                        cnt_slots += 1;
                    }

                    if student.favorites.contains(&student_next) {
                        cnt_favorites += 1;
                    }
                }

                empty_seats.push((i, j, cnt_slots, cnt_favorites));
            }
        }

        empty_seats.sort_by(|a, b| {
            if a.3 == b.3 {
                if a.2 == b.2 {
                    if a.0 == b.0 {
                        a.1.cmp(&b.1)
                    } else {
                        a.0.cmp(&b.0)
                    }
                } else {
                    b.2.cmp(&a.2)
                }
            } else {
                b.3.cmp(&a.3)
            }
        });

        classroom[empty_seats[0].0][empty_seats[0].1] = student.num;
    };

    for student in students.iter() {
        batch_student(&mut classroom, student);
    }

    let calculate_score = |classroom: &Vec<Vec<i64>>| -> i64 {
        let mut ret = 0;

        for i in 0..n {
            for j in 0..n {
                let student = students.iter().find(|x| x.num == classroom[i][j]).unwrap();
                let mut cnt = 0;

                for k in 0..4 {
                    let nx = i as i64 + dx[k];
                    let ny = j as i64 + dy[k];

                    if nx < 0 || nx >= n as i64 || ny < 0 || ny >= n as i64 {
                        continue;
                    }

                    let student_next = classroom[nx as usize][ny as usize];

                    if student.favorites.contains(&student_next) {
                        cnt += 1;
                    }
                }

                match cnt {
                    0 => {}
                    1 => ret += 1,
                    2 => ret += 10,
                    3 => ret += 100,
                    4 => ret += 1000,
                    _ => unreachable!(),
                }
            }
        }

        ret
    };

    writeln!(out, "{}", calculate_score(&classroom)).unwrap();
}
