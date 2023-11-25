use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let k = s.trim().parse::<i64>().unwrap();

    for i in 1..=k {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let n = s.trim().parse::<usize>().unwrap();
        let mut words = vec![String::new(); n];

        for j in 0..n {
            let mut s = String::new();
            io::stdin().read_line(&mut s).unwrap();
            words[j] = s.to_string();
        }

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let abbreviation = s
            .trim()
            .to_string()
            .to_lowercase()
            .chars()
            .collect::<Vec<_>>();

        let mut ret = Vec::new();

        for word in words {
            let word_converted = word.to_lowercase();
            let mut pos = 0;
            let mut is_satisfied = true;

            for ch in abbreviation.iter() {
                match word_converted[pos..].find(*ch) {
                    Some(p) => pos += p + 1,
                    None => {
                        is_satisfied = false;
                        break;
                    }
                }
            }

            if is_satisfied {
                ret.push(word);
            }
        }

        writeln!(out, "Data Set {i}:").unwrap();

        for word in ret {
            write!(out, "{word}").unwrap();
        }
    }
}
