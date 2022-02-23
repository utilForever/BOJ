use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    t = t.trim().to_string();

    let mut lcs_count = vec![vec![0; 1001]; 1001];
    let mut i = 0;
    let mut j;

    let s_chars = s.as_bytes();
    let t_chars = t.as_bytes();
    let s_len = s_chars.len();
    let t_len = t_chars.len();

    for s_idx in 0..s_len {
        i += 1;
        j = 0;

        for t_idx in 0..t_len {
            j += 1;

            if s_chars[s_idx] != t_chars[t_idx] {
                if lcs_count[i - 1][j] > lcs_count[i][j - 1] {
                    lcs_count[i][j] = lcs_count[i - 1][j];
                } else {
                    lcs_count[i][j] = lcs_count[i][j - 1];
                }
            } else {
                lcs_count[i][j] = lcs_count[i - 1][j - 1] + 1;
            }
        }
    }

    let mut lcs_str = String::new();
    let (mut i, mut j) = (s_len, t_len);

    while i != 0 && j != 0 {
        let now = lcs_count[i][j];

        if now != lcs_count[i - 1][j] && now != lcs_count[i][j - 1] {
            lcs_str.push(s_chars[i - 1] as char);
        }

        if now == lcs_count[i - 1][j] {
            i -= 1;
        } else if now == lcs_count[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
            j -= 1;
        }

        if now == 0 {
            break;
        }
    }

    println!("{}", lcs_count[s_len][t_len]);
    println!("{}", lcs_str.chars().rev().collect::<String>());
}
