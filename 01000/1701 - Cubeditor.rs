use std::{cmp, io};

fn main() {
    let mut p = String::new();
    io::stdin().read_line(&mut p).unwrap();

    let p_chars = p.as_bytes();
    let p_len = p_chars.len();

    let mut ret = 0;
    let mut fail = vec![0; 5000];

    for i in 0..p_len {
        let mut cmp = 0;
        fail.fill(0);

        for j in i + 1..p_len {
            while cmp > 0 && p_chars[i + cmp] != p_chars[j] {
                cmp = fail[i + cmp - 1];
            }

            if p_chars[i + cmp] == p_chars[j] {
                cmp += 1;
                fail[j] = cmp;
            }
        }

        ret = cmp::max(ret, *fail.iter().max().unwrap());
    }

    println!("{ret}");
}
