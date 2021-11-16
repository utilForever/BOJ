use std::io;
use io::Write;

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

pub struct Pair {
    pub start: i32,
    pub end: i32,
}

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    // References: https://en.wikipedia.org/wiki/P%C3%B3lya_conjecture
    let polya_list = vec![
        Pair {
            start: 906150257,
            end: 906150258,
        },
        Pair {
            start: 906150259,
            end: 906150293,
        },
        Pair {
            start: 906150295,
            end: 906150307,
        },
        Pair {
            start: 906150311,
            end: 906150313,
        },
        Pair {
            start: 906150315,
            end: 906151515,
        },
        Pair {
            start: 906151517,
            end: 906151575,
        },
        Pair {
            start: 906154583,
            end: 906154585,
        },
        Pair {
            start: 906154605,
            end: 906154605,
        },
        Pair {
            start: 906154609,
            end: 906154757,
        },
        Pair {
            start: 906154763,
            end: 906154763,
        },
        Pair {
            start: 906154769,
            end: 906154769,
        },
        Pair {
            start: 906154789,
            end: 906154789,
        },
        Pair {
            start: 906154791,
            end: 906154791,
        },
        Pair {
            start: 906154793,
            end: 906154793,
        },
        Pair {
            start: 906154825,
            end: 906154825,
        },
        Pair {
            start: 906154829,
            end: 906154829,
        },
        Pair {
            start: 906154837,
            end: 906154837,
        },
        Pair {
            start: 906154857,
            end: 906154857,
        },
        Pair {
            start: 906154865,
            end: 906154881,
        },
        Pair {
            start: 906154885,
            end: 906154885,
        },
        Pair {
            start: 906154887,
            end: 906154887,
        },
        Pair {
            start: 906154889,
            end: 906154889,
        },
        Pair {
            start: 906154891,
            end: 906154891,
        },
        Pair {
            start: 906154893,
            end: 906154893,
        },
        Pair {
            start: 906154895,
            end: 906154907,
        },
        Pair {
            start: 906154909,
            end: 906154911,
        },
        Pair {
            start: 906154915,
            end: 906154927,
        },
        Pair {
            start: 906154947,
            end: 906154949,
        },
        Pair {
            start: 906180359,
            end: 906180361,
        },
        Pair {
            start: 906180363,
            end: 906180363,
        },
        Pair {
            start: 906180365,
            end: 906180365,
        },
        Pair {
            start: 906180367,
            end: 906180369,
        },
        Pair {
            start: 906180371,
            end: 906180373,
        },
        Pair {
            start: 906180375,
            end: 906180375,
        },
        Pair {
            start: 906180391,
            end: 906180517,
        },
        Pair {
            start: 906180519,
            end: 906180519,
        },
        Pair {
            start: 906180525,
            end: 906180533,
        },
        Pair {
            start: 906180537,
            end: 906180553,
        },
        Pair {
            start: 906180555,
            end: 906192697,
        },
        Pair {
            start: 906192847,
            end: 906192865,
        },
        Pair {
            start: 906192867,
            end: 906192903,
        },
        Pair {
            start: 906192905,
            end: 906192905,
        },
        Pair {
            start: 906192907,
            end: 906192965,
        },
        Pair {
            start: 906192971,
            end: 906192971,
        },
        Pair {
            start: 906192979,
            end: 906192983,
        },
        Pair {
            start: 906192985,
            end: 906193227,
        },
        Pair {
            start: 906193229,
            end: 906193233,
        },
        Pair {
            start: 906193245,
            end: 906193245,
        },
        Pair {
            start: 906193247,
            end: 906193247,
        },
        Pair {
            start: 906193303,
            end: 906193303,
        },
        Pair {
            start: 906193419,
            end: 906193419,
        },
        Pair {
            start: 906193465,
            end: 906193465,
        },
        Pair {
            start: 906193475,
            end: 906193475,
        },
        Pair {
            start: 906193477,
            end: 906193477,
        },
        Pair {
            start: 906194931,
            end: 906194931,
        },
        Pair {
            start: 906194933,
            end: 906194945,
        },
        Pair {
            start: 906194949,
            end: 906194949,
        },
        Pair {
            start: 906194951,
            end: 906194967,
        },
        Pair {
            start: 906194979,
            end: 906194979,
        },
        Pair {
            start: 906195099,
            end: 906195099,
        },
        Pair {
            start: 906195109,
            end: 906195149,
        },
        Pair {
            start: 906195151,
            end: 906195151,
        },
        Pair {
            start: 906195297,
            end: 906195297,
        },
        Pair {
            start: 906195299,
            end: 906195985,
        },
        Pair {
            start: 906195989,
            end: 906195989,
        },
        Pair {
            start: 906196009,
            end: 906196009,
        },
        Pair {
            start: 906196011,
            end: 906196013,
        },
        Pair {
            start: 906196015,
            end: 906196015,
        },
        Pair {
            start: 906196045,
            end: 906196051,
        },
        Pair {
            start: 906196053,
            end: 906196055,
        },
        Pair {
            start: 906196057,
            end: 906196071,
        },
        Pair {
            start: 906196077,
            end: 906196079,
        },
        Pair {
            start: 906196081,
            end: 906196081,
        },
        Pair {
            start: 906196083,
            end: 906196091,
        },
        Pair {
            start: 906196099,
            end: 906208711,
        },
        Pair {
            start: 906208713,
            end: 906208713,
        },
        Pair {
            start: 906208731,
            end: 906208731,
        },
        Pair {
            start: 906209041,
            end: 906209063,
        },
        Pair {
            start: 906209067,
            end: 906209067,
        },
        Pair {
            start: 906209069,
            end: 906209069,
        },
        Pair {
            start: 906209071,
            end: 906209223,
        },
        Pair {
            start: 906209227,
            end: 906209227,
        },
        Pair {
            start: 906209233,
            end: 906209235,
        },
        Pair {
            start: 906209237,
            end: 906209237,
        },
        Pair {
            start: 906209241,
            end: 906209241,
        },
        Pair {
            start: 906209243,
            end: 906209271,
        },
        Pair {
            start: 906209283,
            end: 906209283,
        },
        Pair {
            start: 906209285,
            end: 906477701,
        },
        Pair {
            start: 906477735,
            end: 906477811,
        },
        Pair {
            start: 906477867,
            end: 906477867,
        },
        Pair {
            start: 906477869,
            end: 906477869,
        },
        Pair {
            start: 906477871,
            end: 906477899,
        },
        Pair {
            start: 906477901,
            end: 906477901,
        },
        Pair {
            start: 906477903,
            end: 906477905,
        },
        Pair {
            start: 906477929,
            end: 906477931,
        },
        Pair {
            start: 906477933,
            end: 906477933,
        },
        Pair {
            start: 906477935,
            end: 906477935,
        },
        Pair {
            start: 906477937,
            end: 906486639,
        },
        Pair {
            start: 906486805,
            end: 906486805,
        },
        Pair {
            start: 906486807,
            end: 906486807,
        },
        Pair {
            start: 906486817,
            end: 906486817,
        },
        Pair {
            start: 906486819,
            end: 906486819,
        },
        Pair {
            start: 906486821,
            end: 906486831,
        },
        Pair {
            start: 906486843,
            end: 906486853,
        },
        Pair {
            start: 906486855,
            end: 906486855,
        },
        Pair {
            start: 906486909,
            end: 906486913,
        },
        Pair {
            start: 906486917,
            end: 906486973,
        },
        Pair {
            start: 906486975,
            end: 906487001,
        },
        Pair {
            start: 906487005,
            end: 906487063,
        },
        Pair {
            start: 906487065,
            end: 906487065,
        },
        Pair {
            start: 906487069,
            end: 906487069,
        },
        Pair {
            start: 906487071,
            end: 906487071,
        },
        Pair {
            start: 906487073,
            end: 906487077,
        },
        Pair {
            start: 906487085,
            end: 906487085,
        },
        Pair {
            start: 906487087,
            end: 906487101,
        },
        Pair {
            start: 906487185,
            end: 906487185,
        },
        Pair {
            start: 906487187,
            end: 906487189,
        },
        Pair {
            start: 906487191,
            end: 906487191,
        },
        Pair {
            start: 906487193,
            end: 906487193,
        },
        Pair {
            start: 906487195,
            end: 906487203,
        },
        Pair {
            start: 906487205,
            end: 906487205,
        },
        Pair {
            start: 906487259,
            end: 906487259,
        },
        Pair {
            start: 906487261,
            end: 906487261,
        },
        Pair {
            start: 906487263,
            end: 906487263,
        },
        Pair {
            start: 906487271,
            end: 906487287,
        },
        Pair {
            start: 906487933,
            end: 906487933,
        },
        Pair {
            start: 906487937,
            end: 906487937,
        },
        Pair {
            start: 906487949,
            end: 906487973,
        },
        Pair {
            start: 906487975,
            end: 906487993,
        },
        Pair {
            start: 906487995,
            end: 906488001,
        },
        Pair {
            start: 906488003,
            end: 906488003,
        },
        Pair {
            start: 906488007,
            end: 906488007,
        },
        Pair {
            start: 906488009,
            end: 906488009,
        },
        Pair {
            start: 906488023,
            end: 906488025,
        },
        Pair {
            start: 906488027,
            end: 906488065,
        },
        Pair {
            start: 906488067,
            end: 906488067,
        },
        Pair {
            start: 906488077,
            end: 906488079,
        },
    ];

    let t = input_integers()[0];

    for _ in 0..t {
        let n = input_integers()[0];
        let mut is_polya = false;

        for polya in polya_list.iter() {
            if n >= polya.start && n <= polya.end {
                is_polya = true;
                break;
            }
        }

        if n == 1 || is_polya {
            writeln!(out, "E").unwrap();
        } else {
            writeln!(out, "O").unwrap();
        }
    }
}
