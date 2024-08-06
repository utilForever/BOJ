use std::io;

fn input_integers() -> Vec<i32> {
    let mut s = String::new();

    io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    values
}

fn compress(movie: &Vec<Vec<char>>, y_start: usize, y_end: usize, x_start: usize, x_end: usize) {
    let start_color = movie[y_start][x_start];
    let mut is_all_same_color = true;

    for i in y_start..y_end {
        for j in x_start..x_end {
            if movie[i][j] != start_color {
                is_all_same_color = false;
                break;
            }
        }
    }

    if is_all_same_color {
        print!("{}", start_color);
    } else {
        let y_mid = (y_start + y_end) / 2;
        let x_mid = (x_start + x_end) / 2;

        print!("(");

        compress(movie, y_start, y_mid, x_start, x_mid);
        compress(movie, y_start, y_mid, x_mid, x_end);
        compress(movie, y_mid, y_end, x_start, x_mid);
        compress(movie, y_mid, y_end, x_mid, x_end);

        print!(")");
    }
}

fn main() {
    let n = input_integers()[0] as usize;
    let mut movie = vec![vec!['0'; n]; n];

    for i in 0..n {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();
        let mut chars = s.chars();

        for j in 0..n {
            movie[i][j] = chars.next().unwrap();
        }
    }

    compress(&movie, 0, n, 0, n);
}
