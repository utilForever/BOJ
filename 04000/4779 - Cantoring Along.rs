use io::Write;
use std::io;

fn process_cantor(cantor: &mut Vec<char>, start: usize, end: usize, depth: u32) {
    if depth == 0 {
        return;
    }

    let one_third = (end - start) / 3;
    let two_third = one_third * 2;

    for i in start + one_third..start + two_third {
        cantor[i] = ' ';
    }

    process_cantor(cantor, start, start + one_third, depth - 1);
    process_cantor(cantor, start + two_third, end, depth - 1);
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let n = s.parse::<u32>().unwrap();
        let len = 3usize.pow(n);
        let mut cantor = vec!['-'; len];

        process_cantor(&mut cantor, 0, len, n);

        writeln!(out, "{}", cantor.iter().collect::<String>()).unwrap();
    }
}
