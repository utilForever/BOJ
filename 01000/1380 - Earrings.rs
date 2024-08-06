use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut t = 1;
    let mut s = String::new();

    loop {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let n = s.parse::<usize>().unwrap();

        if n == 0 {
            break;
        }

        let mut names = vec![String::new(); n];
        let mut earrings = vec![0; n];

        for i in 0..n {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();

            names[i] = s.trim().to_string();
        }

        for _ in 0..2 * n - 1 {
            s.clear();
            io::stdin().read_line(&mut s).unwrap();
            s = s.trim().to_string();

            let parts = s.split_whitespace().collect::<Vec<_>>();

            let (idx, _) = (
                parts[0].parse::<usize>().unwrap(),
                parts[1].parse::<char>().unwrap(),
            );
            earrings[idx - 1] += 1;
        }

        let pos = earrings.iter().position(|&x| x == 1).unwrap();

        writeln!(out, "{t} {}", names[pos]).unwrap();

        t += 1;
    }
}
