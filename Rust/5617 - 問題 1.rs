use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut num_triangles = 0;
    let mut num_right_angled_triangles = 0;
    let mut num_acute_angled_triangles = 0;
    let mut num_obtuse_triangles = 0;

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let mut lens = s
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        lens.sort();

        if lens[0] + lens[1] <= lens[2] {
            break;
        }

        num_triangles += 1;

        if lens[0].pow(2) + lens[1].pow(2) == lens[2].pow(2) {
            num_right_angled_triangles += 1;
        } else if lens[0].pow(2) + lens[1].pow(2) > lens[2].pow(2) {
            num_acute_angled_triangles += 1;
        } else {
            num_obtuse_triangles += 1;
        }
    }

    writeln!(out, "{num_triangles} {num_right_angled_triangles} {num_acute_angled_triangles} {num_obtuse_triangles}").unwrap();
}
