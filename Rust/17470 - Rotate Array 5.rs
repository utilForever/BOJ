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

#[derive(Debug, Default, Clone, Copy)]
struct Info {
    val: i64,
    flipped_horizontal: bool,
    flipped_vertical: bool,
    rotated: i64,
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut scan = UnsafeScanner::new(stdin.lock());
    let mut out = io::BufWriter::new(stdout.lock());

    let (mut n, mut m, r) = (
        scan.token::<usize>(),
        scan.token::<usize>(),
        scan.token::<i64>(),
    );
    let mut array = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            array[i][j] = scan.token::<i64>();
        }
    }

    let mut divided = vec![Info::default(); 4];
    divided[0].val = 1;
    divided[1].val = 2;
    divided[2].val = 3;
    divided[3].val = 4;

    for _ in 0..r {
        let command = scan.token::<i64>();

        match command {
            1 => {
                // Flip vertically
                divided.swap(0, 2);
                divided.swap(1, 3);

                for i in 0..4 {
                    if divided[i].rotated % 2 == 0 {
                        divided[i].flipped_vertical = !divided[i].flipped_vertical;
                    } else {
                        divided[i].flipped_horizontal = !divided[i].flipped_horizontal;
                    }
                }
            }
            2 => {
                // Flip horizontally
                divided.swap(0, 1);
                divided.swap(2, 3);

                for i in 0..4 {
                    if divided[i].rotated % 2 == 0 {
                        divided[i].flipped_horizontal = !divided[i].flipped_horizontal;
                    } else {
                        divided[i].flipped_vertical = !divided[i].flipped_vertical;
                    }
                }
            }
            3 => {
                // Rotate 90 degrees
                let temp = divided[0];
                divided[0] = divided[2];
                divided[2] = divided[3];
                divided[3] = divided[1];
                divided[1] = temp;

                for i in 0..4 {
                    divided[i].rotated = (divided[i].rotated + 1) % 4;
                }
            }
            4 => {
                // Rotate 270 degrees
                let temp = divided[0];
                divided[0] = divided[1];
                divided[1] = divided[3];
                divided[3] = divided[2];
                divided[2] = temp;

                for i in 0..4 {
                    divided[i].rotated = (divided[i].rotated + 3) % 4;
                }
            }
            5 => {
                let temp = divided[0];
                divided[0] = divided[2];
                divided[2] = divided[3];
                divided[3] = divided[1];
                divided[1] = temp;
            }
            6 => {
                let temp = divided[0];
                divided[0] = divided[1];
                divided[1] = divided[3];
                divided[3] = divided[2];
                divided[2] = temp;
            }
            _ => (),
        }
    }

    let (n_final, m_final) = if divided[0].rotated % 2 == 0 {
        (n, m)
    } else {
        (m, n)
    };
    let mut board_base = vec![vec![0; 101]; 101];
    let mut ret = vec![vec![0; m_final]; n_final];

    for i in 0..4 {
        let y_start = if divided[i].val == 1 || divided[i].val == 2 {
            0
        } else {
            n / 2
        };
        let y_end = if divided[i].val == 1 || divided[i].val == 2 {
            n / 2
        } else {
            n
        };
        let x_start = if divided[i].val == 1 || divided[i].val == 3 {
            0
        } else {
            m / 2
        };
        let x_end = if divided[i].val == 1 || divided[i].val == 3 {
            m / 2
        } else {
            m
        };

        for j in y_start..y_end {
            for k in x_start..x_end {
                // Consider horizontal flip and vertical flip
                let idx_y = if divided[i].flipped_vertical {
                    if divided[i].val == 1 || divided[i].val == 2 {
                        n / 2 - 1 - j
                    } else {
                        n - 1 - j + n / 2
                    }
                } else {
                    j
                };
                let idx_x = if divided[i].flipped_horizontal {
                    if divided[i].val == 1 || divided[i].val == 3 {
                        m / 2 - 1 - k
                    } else {
                        m - 1 - k + m / 2
                    }
                } else {
                    k
                };

                board_base[j][k] = array[idx_y][idx_x];
            }
        }
    }

    for _ in 0..divided[0].rotated {
        let mut temp = vec![vec![0; 101]; 101];

        for i in 0..n {
            for j in 0..m {
                temp[i][j] = board_base[i][j];
            }
        }

        for y in [0, n / 2] {
            for x in [0, m / 2] {
                for i in 0..n / 2 {
                    for j in 0..m / 2 {
                        let idx_y = if y == 0 { 0 } else { m / 2 } + j;
                        let idx_x = if x == 0 { 0 } else { n / 2 } + n / 2 - 1 - i;

                        board_base[idx_y][idx_x] = temp[y + i][x + j];
                    }
                }
            }
        }

        std::mem::swap(&mut n, &mut m);
    }

    for i in 0..4 {
        let offset_ret_y = if i == 0 || i == 1 { 0 } else { n_final / 2 };
        let offset_ret_x = if i == 0 || i == 2 { 0 } else { m_final / 2 };

        for j in 0..n / 2 {
            for k in 0..m / 2 {
                let offset_base_y = if divided[i].val == 1 || divided[i].val == 2 {
                    0
                } else {
                    n / 2
                };
                let offset_base_x = if divided[i].val == 1 || divided[i].val == 3 {
                    0
                } else {
                    m / 2
                };

                ret[j + offset_ret_y][k + offset_ret_x] =
                    board_base[j + offset_base_y][k + offset_base_x];
            }
        }
    }

    for i in 0..n_final {
        for j in 0..m_final {
            write!(out, "{} ", ret[i][j]).unwrap();
        }

        writeln!(out).unwrap();
    }
}
