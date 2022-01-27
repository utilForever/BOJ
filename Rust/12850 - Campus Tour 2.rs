use std::{io, ops};

#[derive(Clone)]
struct Matrix {
    pub elements: [[i64; 8]; 8],
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Matrix {
        let mut ret = Matrix {
            elements: [[0; 8]; 8],
        };

        for i in 0..8 {
            for j in 0..8 {
                for k in 0..8 {
                    ret.elements[i][k] = (ret.elements[i][k]
                        + self.elements[i][j] * rhs.elements[j][k])
                        % 1_000_000_007;
                }
            }
        }

        ret
    }
}

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn calculate_matrix(mut matrix: Matrix, mut end_time: usize) -> Matrix {
    let mut ret = Matrix {
        elements: [[0; 8]; 8],
    };
    for i in 0..8 {
        ret.elements[i][i] = 1;
    }

    while end_time > 0 {
        if end_time % 2 == 1 {
            ret = ret * matrix.clone();
        }

        end_time /= 2;
        matrix = matrix.clone() * matrix.clone();
    }

    ret
}

fn main() {
    let matrix = Matrix {
        elements: [
            [0, 1, 0, 0, 1, 0, 0, 0],
            [1, 0, 1, 0, 1, 0, 0, 0],
            [0, 1, 0, 1, 1, 0, 1, 0],
            [0, 0, 1, 0, 0, 1, 1, 0],
            [1, 1, 1, 0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0, 0, 0, 1],
            [0, 0, 1, 1, 1, 0, 0, 1],
            [0, 0, 0, 0, 0, 1, 1, 0],
        ],
    };

    let d = input_integers()[0] as usize;

    let ret = calculate_matrix(matrix, d);
    println!("{}", ret.elements[0][0]);
}
