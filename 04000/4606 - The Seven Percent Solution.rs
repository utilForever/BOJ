use io::Write;
use std::io;

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s == "#" {
            break;
        }

        s = s.replace("%", "%25");
        s = s.replace("$", "%24");
        s = s.replace(" ", "%20");
        s = s.replace("!", "%21");    
        s = s.replace("(", "%28");
        s = s.replace(")", "%29");
        s = s.replace("*", "%2a");

        writeln!(out, "{s}").unwrap();
    }
}
