use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let t = s.trim().parse::<i64>().unwrap();

    for i in 1..=t {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse::<i64>().unwrap();

        let mut cnt = 0;
        let mut ret = String::new();

        for _ in 0..n {
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().chars().collect::<Vec<_>>();
            let mut alphabet = vec![false; 26];

            for c in name.iter() {
                if !c.is_alphabetic() {
                    continue;
                }

                alphabet[*c as usize - 'A' as usize] = true;
            }

            let cnt_local = alphabet.iter().filter(|&&x| x).count();

            if cnt < cnt_local || (cnt == cnt_local && ret > name.iter().collect::<String>()) {
                cnt = cnt_local;
                ret = name.iter().collect();
            }
        }

        writeln!(out, "Case #{i}: {ret}").unwrap();
    }
}
