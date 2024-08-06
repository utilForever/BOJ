use io::Write;
use std::io;

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

        // (x1, y1, x2, y2, x3, y3, x4, y4)
        let points = s
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<_>>();

        // Case 1: (x1, y1) == (x3, y3)
        // Case 2: (x1, y1) == (x4, y4)
        // Case 3: (x2, y2) == (x3, y3)
        // Case 4: (x2, y2) == (x4, y4)

        let ret = if points[0] == points[4] && points[1] == points[5] {
            let diff = (points[2] - points[0], points[3] - points[1]);
            (points[6] + diff.0, points[7] + diff.1)
        } else if points[0] == points[6] && points[1] == points[7] {
            let diff = (points[2] - points[0], points[3] - points[1]);
            (points[4] + diff.0, points[5] + diff.1)
        } else if points[2] == points[4] && points[3] == points[5] {
            let diff = (points[0] - points[2], points[1] - points[3]);
            (points[6] + diff.0, points[7] + diff.1)
        } else {
            let diff = (points[0] - points[2], points[1] - points[3]);
            (points[4] + diff.0, points[5] + diff.1)
        };

        writeln!(out, "{:.3} {:.3}", ret.0, ret.1).unwrap();
    }
}
