use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();

        let words = s.split_whitespace().collect::<Vec<_>>();
        let len = words.len();

        let mut ret: Vec<String> = Vec::new();
        let mut temp: Vec<String> = Vec::new();
        let mut has_of = false;

        for i in 0..len {
            let mut word = words[i].to_string();

            if word == "of" {
                has_of = true;
            } else if word.starts_with("Korea") {
                if has_of
                    && (word.len() == 5 || !word.chars().nth(5).unwrap().is_alphabetic())
                    && temp.len() >= 2
                    && temp[temp.len() - 2].chars().last().unwrap().is_alphabetic()
                {
                    let mut has_punctuation = false;
                    let mut punctuation = None;

                    if word.len() > 5 {
                        has_punctuation = true;
                        punctuation = Some(word.chars().nth(5).unwrap());
                    }

                    temp.pop();
                    word = temp.pop().unwrap();

                    word.insert_str(0, "K-");

                    if has_punctuation {
                        word.push(punctuation.unwrap());
                    }

                    let connector = word[2..3].chars().next().unwrap();

                    if connector.is_ascii_lowercase() {
                        word.replace_range(2..3, &connector.to_uppercase().to_string());
                    }
                }

                has_of = false;
                temp.push(word);
                continue;
            } else {
                has_of = false;
            }

            temp.push(word);
        }

        let len = temp.len();

        for i in (0..len).rev() {
            let mut s = temp[i].to_string();

            if s == "Korea" {
                if ret.is_empty() {
                    ret.push(s);
                    continue;
                }

                s = ret.pop().unwrap();
                s.insert_str(0, "K-");

                let connector = s[2..3].chars().next().unwrap();

                if connector.is_ascii_lowercase() {
                    s.replace_range(2..3, &connector.to_uppercase().to_string());
                }
            }

            ret.push(s);
        }

        ret.reverse();

        for val in ret {
            write!(out, "{val} ").unwrap();
        }

        writeln!(out).unwrap();
    }
}
