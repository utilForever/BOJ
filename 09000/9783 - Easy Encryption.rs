use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s = s.trim().to_string();
    let s = s.chars().collect::<Vec<_>>();

    for c in s.iter() {
        if c.is_numeric() {
            write!(out, "#{c}").unwrap();
        } else if c.is_ascii_alphabetic() {
            write!(
                out,
                "{}",
                match c {
                    'a' => "01",
                    'b' => "02",
                    'c' => "03",
                    'd' => "04",
                    'e' => "05",
                    'f' => "06",
                    'g' => "07",
                    'h' => "08",
                    'i' => "09",
                    'j' => "10",
                    'k' => "11",
                    'l' => "12",
                    'm' => "13",
                    'n' => "14",
                    'o' => "15",
                    'p' => "16",
                    'q' => "17",
                    'r' => "18",
                    's' => "19",
                    't' => "20",
                    'u' => "21",
                    'v' => "22",
                    'w' => "23",
                    'x' => "24",
                    'y' => "25",
                    'z' => "26",
                    'A' => "27",
                    'B' => "28",
                    'C' => "29",
                    'D' => "30",
                    'E' => "31",
                    'F' => "32",
                    'G' => "33",
                    'H' => "34",
                    'I' => "35",
                    'J' => "36",
                    'K' => "37",
                    'L' => "38",
                    'M' => "39",
                    'N' => "40",
                    'O' => "41",
                    'P' => "42",
                    'Q' => "43",
                    'R' => "44",
                    'S' => "45",
                    'T' => "46",
                    'U' => "47",
                    'V' => "48",
                    'W' => "49",
                    'X' => "50",
                    'Y' => "51",
                    'Z' => "52",
                    _ => unreachable!(),
                }
            )
            .unwrap();
        } else {
            write!(out, "{c}").unwrap();
        }
    }

    writeln!(out).unwrap();
}
