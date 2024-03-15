use io::Write;
use std::io;

fn process0() -> Vec<String> {
    // Process 0
    Vec::new()
}

fn process1() -> Vec<String> {
    // Process 1
    Vec::new()
}

fn process2() -> Vec<String> {
    // Process 2
    Vec::new()
}

fn process3() -> Vec<String> {
    // Process 3
    Vec::new()
}

fn process4() -> Vec<String> {
    // Process 4
    Vec::new()
}

fn process5() -> Vec<String> {
    // Process 5
    Vec::new()
}

fn process6() -> Vec<String> {
    // Process 6
    Vec::new()
}

fn process7() -> Vec<String> {
    // Process 7
    Vec::new()
}

fn process8() -> Vec<String> {
    // Process 8
    Vec::new()
}

fn process9() -> Vec<String> {
    // Process 9
    Vec::new()
}

// ~30000~
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n = s.trim().parse::<i64>().unwrap();

    let ret = match n {
        0 => process0(),
        1 => process1(),
        2 => process2(),
        3 => process3(),
        4 => process4(),
        5 => process5(),
        6 => process6(),
        7 => process7(),
        8 => process8(),
        9 => process9(),
        _ => unreachable!(),
    };

    for s in ret {
        writeln!(out, "{s}").ok();
    }
}
