use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();
    let s = s.split_whitespace().collect::<Vec<_>>();

    let mut ret = String::new();

    for (idx, &word) in s.iter().enumerate() {
        if idx != 0
            && (word == "i"
                || word == "pa"
                || word == "te"
                || word == "ni"
                || word == "niti"
                || word == "a"
                || word == "ali"
                || word == "nego"
                || word == "no"
                || word == "ili")
        {
            continue;
        }

        ret.push_str(word[0..1].to_string().as_str());
    }

    writeln!(out, "{}", ret.to_uppercase()).unwrap();
}
