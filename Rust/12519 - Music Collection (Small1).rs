use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let t = s.parse::<i64>().unwrap();

    for i in 1..=t {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let n = s.parse::<usize>().unwrap();
        let mut songs = vec![String::new(); n];

        for j in 0..n {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();
            songs[j] = s.trim().to_string().to_uppercase();
        }

        writeln!(out, "Case #{i}:").unwrap();

        if songs.len() == 1 {
            writeln!(out, "\"\"").unwrap();
            continue;
        }

        for j in 0..songs.len() {
            let mut ret = String::new();

            for k in 1..=songs[j].len() {
                for l in 0..=songs[j].len() - k {
                    let substring = &songs[j][l..l + k];
                    let mut is_contain = false;

                    for m in 0..songs.len() {
                        if m == j {
                            continue;
                        }

                        if songs[m].contains(substring) {
                            is_contain = true;
                            break;
                        }
                    }

                    if !is_contain {
                        if ret.is_empty() {
                            ret = substring.to_string();
                        } else {
                            ret = ret.min(substring.to_string());
                        }
                    }
                }

                if !ret.is_empty() {
                    break;
                }
            }

            if ret.is_empty() {
                writeln!(out, ":(").unwrap();
            } else {
                writeln!(out, "\"{ret}\"").unwrap();
            }
        }
    }
}
