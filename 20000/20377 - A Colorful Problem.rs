use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    let mut colors_target = [(0, 0, 0); 16];

    for i in 0..16 {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let colors = s.split_whitespace().collect::<Vec<_>>();

        colors_target[i] = (
            colors[0].parse::<i64>().unwrap(),
            colors[1].parse::<i64>().unwrap(),
            colors[2].parse::<i64>().unwrap(),
        );
    }

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let color = s.split_whitespace().collect::<Vec<_>>();

        if color.is_empty() {
            break;
        }

        let color = (
            color[0].parse::<i64>().unwrap(),
            color[1].parse::<i64>().unwrap(),
            color[2].parse::<i64>().unwrap(),
        );

        if color == (-1, -1, -1) {
            break;
        }

        let mut ret = f64::MAX;
        let mut ret_color = (0, 0, 0);

        for color_target in colors_target.iter() {
            let dist = ((color_target.0 - color.0).pow(2)
                + (color_target.1 - color.1).pow(2)
                + (color_target.2 - color.2).pow(2)) as f64;

            if dist < ret {
                ret = dist;
                ret_color = *color_target;
            }
        }

        writeln!(
            out,
            "{:>3} {:>3} {:>3} maps to {:>3} {:>3} {:>3}",
            color.0, color.1, color.2, ret_color.0, ret_color.1, ret_color.2
        )
        .unwrap();
    }
}
