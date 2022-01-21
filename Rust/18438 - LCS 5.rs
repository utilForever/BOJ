use std::{cmp, io};

static mut LCS1: Vec<Vec<i32>> = Vec::new();
static mut LCS2: Vec<Vec<i32>> = Vec::new();
static mut S: String = String::new();
static mut T: String = String::new();

// Reference: https://www.secmem.org/blog/2019/12/15/dpopt-ch2/
unsafe fn process_lcs_with_hirschburg(s1: usize, s2: usize, t1: usize, t2: usize) -> String {
    if s1 > s2 {
        return String::new();
    }

    let mut ret = String::new();

    if s1 == s2 {
        for i in t1..=t2 {
            if S.chars().nth(s2).unwrap() == T.chars().nth(i).unwrap() {
                ret.push(T.chars().nth(i).unwrap());
                return ret;
            }
        }

        return String::new();
    }

    let mid_s = (s1 + s2) / 2;

    for i in t1..=(t2 + 2) {
        LCS1[0][i] = 0;
        LCS1[1][i] = 0;
        LCS2[0][i] = 0;
        LCS2[1][i] = 0;
    }

    for i in s1..=mid_s {
        for j in t1..=t2 {
            if S.chars().nth(i).unwrap() == T.chars().nth(j).unwrap() {
                LCS1[i % 2][j + 1] = LCS1[(i + 1) % 2][j] + 1;
            } else {
                LCS1[i % 2][j + 1] = cmp::max(LCS1[(i + 1) % 2][j + 1], LCS1[i % 2][j]);
            }
        }
    }

    for i in ((mid_s + 1)..=s2).rev() {
        for j in (t1..=t2).rev() {
            if S.chars().nth(i).unwrap() == T.chars().nth(j).unwrap() {
                LCS2[i % 2][j + 1] = LCS2[(i + 1) % 2][j + 2] + 1;
            } else {
                LCS2[i % 2][j + 1] = cmp::max(LCS2[(i + 1) % 2][j + 1], LCS2[i % 2][j + 2]);
            }
        }
    }

    let mut max = -1;
    let mut mid_t = 0;

    for i in t1..=(t2 + 1) {
        if LCS1[mid_s % 2][i] + LCS2[mid_s % 2][i + 1] > max
        {
            max = LCS1[mid_s % 2][i] + LCS2[mid_s % 2][i + 1];
            mid_t = i;
        }
    }

    process_lcs_with_hirschburg(s1, mid_s, t1, mid_t - 1)
        + &process_lcs_with_hirschburg(mid_s + 1, s2, mid_t, t2)
}

fn main() {
    unsafe {
        LCS1.resize(2, Vec::new());
        LCS2.resize(2, Vec::new());

        for i in 0..2 {
            LCS1[i].resize(7001, 0);
            LCS2[i].resize(7001, 0);
        }

        io::stdin().read_line(&mut S).unwrap();
        S = S.trim().to_string();

        io::stdin().read_line(&mut T).unwrap();
        T = T.trim().to_string();

        let lcs = process_lcs_with_hirschburg(0, S.len() - 1, 0, T.len() - 1);

        println!("{}", lcs.len());
        println!("{}", lcs);
    }
}
