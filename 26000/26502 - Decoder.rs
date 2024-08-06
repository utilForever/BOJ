use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        let words = s.trim().to_string();
        let mut words = words.chars().collect::<Vec<_>>();

        for i in 0..words.len() {
            words[i] = match words[i] {
                'y' => 'a',
                'a' => 'e',
                'e' => 'i',
                'i' => 'o',
                'o' => 'u',
                'u' => 'y',
                'Y' => 'A',
                'A' => 'E',
                'E' => 'I',
                'I' => 'O',
                'O' => 'U',
                'U' => 'Y',
                _ => words[i],
            };
        }

        writeln!(out, "{}", words.iter().collect::<String>()).unwrap();
    }
}
