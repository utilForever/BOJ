use io::Write;
use std::{collections::VecDeque, io};

fn input_integers() -> Vec<i64> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i64> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let t = input_integers()[0] as usize;

    for _ in 0..t {
        let mut p = String::new();
        io::stdin().read_line(&mut p).unwrap();
        let _ = input_integers()[0] as usize;
        let mut arr = String::new();
        io::stdin().read_line(&mut arr).unwrap();

        let mut deque = VecDeque::new();
        let mut chars = arr.chars();

        while let Some(c) = chars.next() {
            if c >= '0' && c <= '9' {
                let c2 = chars.next().unwrap();

                if c2 >= '0' && c2 <= '9' {
                    let c3 = chars.next().unwrap();

                    if c3 == '0' {
                        deque.push_back(100);
                    } else {
                        deque.push_back((c as u8 - '0' as u8) * 10 + c2 as u8 - '0' as u8);
                    }
                } else {
                    deque.push_back(c as u8 - '0' as u8);
                }
            }
        }

        let mut is_error = false;
        let mut is_front = true;

        for func in p.chars() {
            if func == 'R' {
                is_front = !is_front;
            } else if func == 'D' {
                if deque.is_empty() {
                    is_error = true;

                    writeln!(out, "error").unwrap();
                    break;
                }

                if is_front {
                    deque.pop_front();
                } else {
                    deque.pop_back();
                }
            }
        }

        let mut num_elems = 0;

        if !is_error {
            write!(out, "[").unwrap();

            if is_front {
                for elem in deque.iter() {
                    write!(out, "{}", elem).unwrap();

                    if num_elems != deque.len() - 1 {
                        write!(out, ",").unwrap();
                    }

                    num_elems += 1;
                }
            } else {
                for elem in deque.iter().rev() {
                    write!(out, "{}", elem).unwrap();

                    if num_elems != deque.len() - 1 {
                        write!(out, ",").unwrap();
                    }

                    num_elems += 1;
                }
            }

            writeln!(out, "]").unwrap();
        }
    }
}
