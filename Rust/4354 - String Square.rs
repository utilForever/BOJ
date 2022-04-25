use std::io;

fn main() {
    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        if s.trim() == "." {
            break;
        }

        let s_chars = s.trim().as_bytes();
        let s_len = s_chars.len();

        let mut cmp = 0;
        let mut fail = vec![0; 1_000_000];

        for i in 1..s_len {
            while cmp > 0 && s_chars[cmp] != s_chars[i] {
                cmp = fail[cmp - 1];
            }

            if s_chars[cmp] == s_chars[i] {
                cmp += 1;
                fail[i] = cmp;
            }
        }

        println!(
            "{}",
            if s_len % (s_len - fail[s_len - 1]) > 0 {
                1
            } else {
                s_len / (s_len - fail[s_len - 1])
            }
        );
    }
}
