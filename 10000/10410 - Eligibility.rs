use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim().to_string();

    let n = s.parse::<i64>().unwrap();

    for _ in 0..n {
        s.clear();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let data = s.split(" ").collect::<Vec<&str>>();

        let (name, date_study, date_birth, courses) = (data[0], data[1], data[2], data[3]);
        let date_study = (
            date_study[0..4].parse::<i64>().unwrap(),
            date_study[5..7].parse::<i64>().unwrap(),
            date_study[8..10].parse::<i64>().unwrap(),
        );
        let date_birth = (
            date_birth[0..4].parse::<i64>().unwrap(),
            date_birth[5..7].parse::<i64>().unwrap(),
            date_birth[8..10].parse::<i64>().unwrap(),
        );
        let courses = courses.parse::<i64>().unwrap();

        if date_study.0 >= 2010 || date_birth.0 >= 1991 {
            writeln!(out, "{name} eligible").unwrap();
        } else if courses >= 41 {
            writeln!(out, "{name} ineligible").unwrap();
        } else {
            writeln!(out, "{name} coach petitions").unwrap();
        }
    }
}
