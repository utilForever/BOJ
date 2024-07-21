use io::Write;
use std::io;

fn convert_char_to_num(c: char) -> i64 {
    match c {
        ' ' => 1,
        'A' | 'B' | 'C' => 2,
        'D' | 'E' | 'F' => 3,
        'G' | 'H' | 'I' => 4,
        'J' | 'K' | 'L' => 5,
        'M' | 'N' | 'O' => 6,
        'P' | 'Q' | 'R' | 'S' => 7,
        'T' | 'U' | 'V' => 8,
        'W' | 'X' | 'Y' | 'Z' => 9,
        _ => unreachable!(),
    }
}

fn convert_char_to_count(c: char) -> i64 {
    match c {
        ' ' => 1,
        'A' | 'D' | 'G' | 'J' | 'M' | 'P' | 'T' | 'W' => 1,
        'B' | 'E' | 'H' | 'K' | 'N' | 'Q' | 'U' | 'X' => 2,
        'C' | 'F' | 'I' | 'L' | 'O' | 'R' | 'V' | 'Y' => 3,
        'S' | 'Z' => 4,
        _ => unreachable!(),
    }

}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let (p, w) = s.split_at(s.find(' ').unwrap());
    let (p, w) = (p.parse::<i64>().unwrap(), w.trim().parse::<i64>().unwrap());

    s.clear();
    io::stdin().read_line(&mut s).unwrap();

    let s = s.trim().to_string().chars().collect::<Vec<_>>();
    let mut prev = convert_char_to_num(s[0]);
    let mut ret = p * convert_char_to_count(s[0]);

    for i in 1..s.len() {
        let curr = convert_char_to_num(s[i]);

        if prev == curr && curr != 1 {
            ret += w;
        }

        ret += p * convert_char_to_count(s[i]);
        prev = curr;
    }

    writeln!(out, "{ret}").unwrap();
}
