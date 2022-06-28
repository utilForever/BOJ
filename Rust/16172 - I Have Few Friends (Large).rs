use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let mut k = String::new();
    io::stdin().read_line(&mut k).unwrap();

    let s = s
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<String>();
    let k = k.trim().to_string();

    let s_chars = s.as_bytes();
    let s_len = s_chars.len();
    let k_chars = k.as_bytes();
    let k_len = k_chars.len();

    let mut cmp = 0;
    let mut fail = vec![0; 200_000];

    for i in 1..k_len {
        while cmp > 0 && k_chars[cmp] != k_chars[i] {
            cmp = fail[cmp - 1];
        }

        if k_chars[cmp] == k_chars[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    cmp = 0;

    for i in 0..s_len {
        while cmp > 0 && s_chars[i] != k_chars[cmp] {
            cmp = fail[cmp - 1];
        }

        if s_chars[i] == k_chars[cmp] {
            if cmp == k_len - 1 {
                println!("1");
                return;
            } else {
                cmp += 1;
            }
        }
    }

    println!("0");
}
