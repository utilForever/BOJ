use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let n = s.trim().parse::<i64>().unwrap();
    let mut subjects = vec![vec![Vec::new(); 3]; 4];

    for _ in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let subject_temp = s.split_whitespace().collect::<Vec<_>>();
        let mut subject: Vec<String> = Vec::new();
        let mut idx = 0;

        subject.push(String::new());

        for s in subject_temp {
            if subject[idx].is_empty() {
                subject[idx].push_str(s);
            } else if subject[idx].len() + 1 + s.len() > 10 {
                idx += 1;
                subject.push(String::new());
                subject[idx].push_str(s);
            } else {
                subject[idx].push(' ');
                subject[idx].push_str(s);
            }
        }

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        let words = s.split_whitespace().collect::<Vec<_>>();
        let (day, lesson) = (words[0], words[1]);

        let i = match lesson {
            "1" => 0,
            "2" => 1,
            "3" => 2,
            "4" => 3,
            _ => unreachable!(),
        };
        let j = match day {
            "Tuesday" => 0,
            "Thursday" => 1,
            "Saturday" => 2,
            _ => unreachable!(),
        };

        subjects[i][j] = subject;
    }

    writeln!(out, "+----------+----------+----------+").unwrap();

    for i in 0..4 {
        let cnt_line = subjects[i]
            .iter()
            .map(|subject| subject.len())
            .max()
            .unwrap();

        if cnt_line == 0 {
            write!(out, "|").unwrap();

            for _ in 0..3 {
                for _ in 0..10 {
                    write!(out, " ").unwrap();
                }

                write!(out, "|").unwrap();
            }

            writeln!(out).unwrap();
        }

        for j in 0..cnt_line {
            write!(out, "|").unwrap();

            for k in 0..3 {
                let len = subjects[i][k].len();

                if len == 0 {
                    for _ in 0..10 {
                        write!(out, " ").unwrap();
                    }
                } else {
                    if j >= len {
                        for _ in 0..10 {
                            write!(out, " ").unwrap();
                        }
                    } else {
                        let len = subjects[i][k][j].len();

                        write!(out, "{}", subjects[i][k][j]).unwrap();

                        for _ in 0..10 - len {
                            write!(out, " ").unwrap();
                        }
                    }
                }

                write!(out, "|").unwrap();
            }

            writeln!(out).unwrap();
        }

        writeln!(out, "+----------+----------+----------+").unwrap();
    }
}
