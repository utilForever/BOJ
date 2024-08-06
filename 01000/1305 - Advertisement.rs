use std::io;

fn main() {
	let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();
	let l = l.trim().parse::<usize>().unwrap();

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();

    let s_chars = s.as_bytes();

    let mut cmp = 0;
    let mut fail = vec![0; 1_000_000];

    for i in 1..l {
        while cmp > 0 && s_chars[cmp] != s_chars[i] {
            cmp = fail[cmp - 1];
        }

        if s_chars[cmp] == s_chars[i] {
            cmp += 1;
            fail[i] = cmp;
        }
    }

    println!("{}", l - fail[l - 1]);
}
