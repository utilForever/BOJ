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

    let t = scan.token::<i64>();

    for _ in 0..t {
        let n = scan.token::<i64>();

        /*
                    01 02 03
                    04 05 06
                    07 08 09
           28 29 30 10 11 12 37 38 39 46 47 48
           31 32 33 13 14 15 40 41 42 49 50 51
           34 35 36 16 17 18 43 44 45 52 53 54
                    19 20 21
                    22 23 24
                    25 26 27
        */

        let mut cube = vec![' '; 3 * 3 * 6 + 1];

        for i in 1..=54 {
            cube[i] = match i {
                1..=9 => 'w',
                10..=18 => 'r',
                19..=27 => 'y',
                28..=36 => 'g',
                37..=45 => 'b',
                46..=54 => 'o',
                _ => unreachable!(),
            };
        }

        for _ in 0..n {
            let command = scan.token::<String>().chars().collect::<Vec<_>>();
            let (face, dir) = (command[0], command[1]);
            let mut cube_new = cube.clone();

            match face {
                'U' => {
                    if dir == '+' {
                        cube_new[10] = cube[37];
                        cube_new[11] = cube[38];
                        cube_new[12] = cube[39];

                        cube_new[28] = cube[10];
                        cube_new[29] = cube[11];
                        cube_new[30] = cube[12];

                        cube_new[37] = cube[46];
                        cube_new[38] = cube[47];
                        cube_new[39] = cube[48];

                        cube_new[46] = cube[28];
                        cube_new[47] = cube[29];
                        cube_new[48] = cube[30];

                        cube_new[1] = cube[3];
                        cube_new[2] = cube[6];
                        cube_new[3] = cube[9];
                        cube_new[4] = cube[2];
                        cube_new[5] = cube[5];
                        cube_new[6] = cube[8];
                        cube_new[7] = cube[1];
                        cube_new[8] = cube[4];
                        cube_new[9] = cube[7];
                    } else {
                        cube_new[10] = cube[28];
                        cube_new[11] = cube[29];
                        cube_new[12] = cube[30];

                        cube_new[28] = cube[46];
                        cube_new[29] = cube[47];
                        cube_new[30] = cube[48];

                        cube_new[37] = cube[10];
                        cube_new[38] = cube[11];
                        cube_new[39] = cube[12];

                        cube_new[46] = cube[37];
                        cube_new[47] = cube[38];
                        cube_new[48] = cube[39];

                        cube_new[1] = cube[7];
                        cube_new[2] = cube[4];
                        cube_new[3] = cube[1];
                        cube_new[4] = cube[8];
                        cube_new[5] = cube[5];
                        cube_new[6] = cube[2];
                        cube_new[7] = cube[9];
                        cube_new[8] = cube[6];
                        cube_new[9] = cube[3];
                    }
                }
                'D' => {
                    if dir == '+' {
                        cube_new[16] = cube[34];
                        cube_new[17] = cube[35];
                        cube_new[18] = cube[36];

                        cube_new[34] = cube[52];
                        cube_new[35] = cube[53];
                        cube_new[36] = cube[54];

                        cube_new[43] = cube[16];
                        cube_new[44] = cube[17];
                        cube_new[45] = cube[18];

                        cube_new[52] = cube[43];
                        cube_new[53] = cube[44];
                        cube_new[54] = cube[45];

                        cube_new[19] = cube[21];
                        cube_new[20] = cube[24];
                        cube_new[21] = cube[27];
                        cube_new[22] = cube[20];
                        cube_new[23] = cube[23];
                        cube_new[24] = cube[26];
                        cube_new[25] = cube[19];
                        cube_new[26] = cube[22];
                        cube_new[27] = cube[25];
                    } else {
                        cube_new[16] = cube[43];
                        cube_new[17] = cube[44];
                        cube_new[18] = cube[45];

                        cube_new[34] = cube[16];
                        cube_new[35] = cube[17];
                        cube_new[36] = cube[18];

                        cube_new[43] = cube[52];
                        cube_new[44] = cube[53];
                        cube_new[45] = cube[54];

                        cube_new[52] = cube[34];
                        cube_new[53] = cube[35];
                        cube_new[54] = cube[36];

                        cube_new[19] = cube[25];
                        cube_new[20] = cube[22];
                        cube_new[21] = cube[19];
                        cube_new[22] = cube[26];
                        cube_new[23] = cube[23];
                        cube_new[24] = cube[20];
                        cube_new[25] = cube[27];
                        cube_new[26] = cube[24];
                        cube_new[27] = cube[21];
                    }
                }
                'F' => {
                    if dir == '+' {
                        cube_new[1] = cube[34];
                        cube_new[2] = cube[31];
                        cube_new[3] = cube[28];

                        cube_new[25] = cube[45];
                        cube_new[26] = cube[42];
                        cube_new[27] = cube[39];

                        cube_new[28] = cube[25];
                        cube_new[31] = cube[26];
                        cube_new[34] = cube[27];

                        cube_new[39] = cube[1];
                        cube_new[42] = cube[2];
                        cube_new[45] = cube[3];

                        cube_new[10] = cube[12];
                        cube_new[11] = cube[15];
                        cube_new[12] = cube[18];
                        cube_new[13] = cube[11];
                        cube_new[14] = cube[14];
                        cube_new[15] = cube[17];
                        cube_new[16] = cube[10];
                        cube_new[17] = cube[13];
                        cube_new[18] = cube[16];
                    } else {
                        cube_new[1] = cube[39];
                        cube_new[2] = cube[42];
                        cube_new[3] = cube[45];

                        cube_new[25] = cube[28];
                        cube_new[26] = cube[31];
                        cube_new[27] = cube[34];

                        cube_new[28] = cube[3];
                        cube_new[31] = cube[2];
                        cube_new[34] = cube[1];

                        cube_new[39] = cube[27];
                        cube_new[42] = cube[26];
                        cube_new[45] = cube[25];

                        cube_new[10] = cube[16];
                        cube_new[11] = cube[13];
                        cube_new[12] = cube[10];
                        cube_new[13] = cube[17];
                        cube_new[14] = cube[14];
                        cube_new[15] = cube[11];
                        cube_new[16] = cube[18];
                        cube_new[17] = cube[15];
                        cube_new[18] = cube[12];
                    }
                }
                'B' => {
                    if dir == '+' {
                        cube_new[7] = cube[37];
                        cube_new[8] = cube[40];
                        cube_new[9] = cube[43];

                        cube_new[19] = cube[30];
                        cube_new[20] = cube[33];
                        cube_new[21] = cube[36];

                        cube_new[30] = cube[9];
                        cube_new[33] = cube[8];
                        cube_new[36] = cube[7];

                        cube_new[37] = cube[21];
                        cube_new[40] = cube[20];
                        cube_new[43] = cube[19];

                        cube_new[46] = cube[48];
                        cube_new[47] = cube[51];
                        cube_new[48] = cube[54];
                        cube_new[49] = cube[47];
                        cube_new[50] = cube[50];
                        cube_new[51] = cube[53];
                        cube_new[52] = cube[46];
                        cube_new[53] = cube[49];
                        cube_new[54] = cube[52];
                    } else {
                        cube_new[7] = cube[36];
                        cube_new[8] = cube[33];
                        cube_new[9] = cube[30];

                        cube_new[19] = cube[43];
                        cube_new[20] = cube[40];
                        cube_new[21] = cube[37];

                        cube_new[30] = cube[19];
                        cube_new[33] = cube[20];
                        cube_new[36] = cube[21];

                        cube_new[37] = cube[7];
                        cube_new[40] = cube[8];
                        cube_new[43] = cube[9];

                        cube_new[46] = cube[52];
                        cube_new[47] = cube[49];
                        cube_new[48] = cube[46];
                        cube_new[49] = cube[53];
                        cube_new[50] = cube[50];
                        cube_new[51] = cube[47];
                        cube_new[52] = cube[54];
                        cube_new[53] = cube[51];
                        cube_new[54] = cube[48];
                    }
                }
                'L' => {
                    if dir == '+' {
                        cube_new[1] = cube[46];
                        cube_new[4] = cube[49];
                        cube_new[7] = cube[52];

                        cube_new[12] = cube[7];
                        cube_new[15] = cube[4];
                        cube_new[18] = cube[1];

                        cube_new[19] = cube[18];
                        cube_new[22] = cube[15];
                        cube_new[25] = cube[12];

                        cube_new[46] = cube[19];
                        cube_new[49] = cube[22];
                        cube_new[52] = cube[25];

                        cube_new[28] = cube[30];
                        cube_new[29] = cube[33];
                        cube_new[30] = cube[36];
                        cube_new[31] = cube[29];
                        cube_new[32] = cube[32];
                        cube_new[33] = cube[35];
                        cube_new[34] = cube[28];
                        cube_new[35] = cube[31];
                        cube_new[36] = cube[34];
                    } else {
                        cube_new[1] = cube[18];
                        cube_new[4] = cube[15];
                        cube_new[7] = cube[12];

                        cube_new[12] = cube[25];
                        cube_new[15] = cube[22];
                        cube_new[18] = cube[19];

                        cube_new[19] = cube[46];
                        cube_new[22] = cube[49];
                        cube_new[25] = cube[52];

                        cube_new[46] = cube[1];
                        cube_new[49] = cube[4];
                        cube_new[52] = cube[7];

                        cube_new[28] = cube[34];
                        cube_new[29] = cube[31];
                        cube_new[30] = cube[28];
                        cube_new[31] = cube[35];
                        cube_new[32] = cube[32];
                        cube_new[33] = cube[29];
                        cube_new[34] = cube[36];
                        cube_new[35] = cube[33];
                        cube_new[36] = cube[30];
                    }
                }
                'R' => {
                    if dir == '+' {
                        cube_new[3] = cube[16];
                        cube_new[6] = cube[13];
                        cube_new[9] = cube[10];

                        cube_new[10] = cube[27];
                        cube_new[13] = cube[24];
                        cube_new[16] = cube[21];

                        cube_new[21] = cube[48];
                        cube_new[24] = cube[51];
                        cube_new[27] = cube[54];

                        cube_new[48] = cube[3];
                        cube_new[51] = cube[6];
                        cube_new[54] = cube[9];

                        cube_new[37] = cube[39];
                        cube_new[38] = cube[42];
                        cube_new[39] = cube[45];
                        cube_new[40] = cube[38];
                        cube_new[41] = cube[41];
                        cube_new[42] = cube[44];
                        cube_new[43] = cube[37];
                        cube_new[44] = cube[40];
                        cube_new[45] = cube[43];
                    } else {
                        cube_new[3] = cube[48];
                        cube_new[6] = cube[51];
                        cube_new[9] = cube[54];

                        cube_new[10] = cube[9];
                        cube_new[13] = cube[6];
                        cube_new[16] = cube[3];

                        cube_new[21] = cube[16];
                        cube_new[24] = cube[13];
                        cube_new[27] = cube[10];

                        cube_new[48] = cube[21];
                        cube_new[51] = cube[24];
                        cube_new[54] = cube[27];

                        cube_new[37] = cube[43];
                        cube_new[38] = cube[40];
                        cube_new[39] = cube[37];
                        cube_new[40] = cube[44];
                        cube_new[41] = cube[41];
                        cube_new[42] = cube[38];
                        cube_new[43] = cube[45];
                        cube_new[44] = cube[42];
                        cube_new[45] = cube[39];
                    }
                }
                _ => unreachable!(),
            }

            cube = cube_new;
        }

        writeln!(out, "{}{}{}", cube[7], cube[8], cube[9]).unwrap();
        writeln!(out, "{}{}{}", cube[4], cube[5], cube[6]).unwrap();
        writeln!(out, "{}{}{}", cube[1], cube[2], cube[3]).unwrap();
    }
}
