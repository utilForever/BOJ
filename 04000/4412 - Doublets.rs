use io::Write;
use std::{
    collections::{HashMap, VecDeque},
    io,
};

fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut words: HashMap<usize, Vec<String>> = HashMap::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        s = s.trim().to_string();

        if s.is_empty() {
            break;
        }

        let len = s.len();
        words.entry(len).or_default().push(s);
    }

    let mut adj_list_words: HashMap<usize, HashMap<String, Vec<String>>> = HashMap::new();

    for (&len, words) in words.iter() {
        let mut adj_list: HashMap<String, Vec<String>> = HashMap::new();
        let mut buckets: HashMap<Vec<u8>, Vec<String>> = HashMap::new();

        // Create buckets for words with one character replaced by a wildcard
        for word in words {
            for i in 0..len {
                let mut bucket = word.clone().into_bytes();
                bucket[i] = b'*';

                buckets
                    .entry(bucket)
                    .or_insert(Vec::new())
                    .push(word.clone());
            }
        }

        // Build adjacency list using the buckets
        for bucket in buckets.values() {
            for i in 0..bucket.len() {
                for j in i + 1..bucket.len() {
                    let word1 = &bucket[i];
                    let word2 = &bucket[j];

                    adj_list
                        .entry(word1.clone())
                        .or_default()
                        .push(word2.clone());
                    adj_list
                        .entry(word2.clone())
                        .or_default()
                        .push(word1.clone());
                }
            }
        }

        adj_list_words.insert(len, adj_list);
    }

    let mut t = 0;

    loop {
        let mut word_pair = String::new();
        io::stdin().read_line(&mut word_pair).unwrap();
        word_pair = word_pair.trim().to_string();

        if word_pair.is_empty() {
            break;
        }

        if t > 0 {
            writeln!(out).unwrap();
        }

        let word_pair = word_pair.split_whitespace().collect::<Vec<_>>();
        let (s, e) = (word_pair[0], word_pair[1]);

        if s.len() != e.len() {
            writeln!(out, "No solution.").unwrap();
            continue;
        }

        let adj_list = &adj_list_words[&s.len()];
        if !adj_list.contains_key(s) || !adj_list.contains_key(e) {
            writeln!(out, "No solution.").unwrap();
            continue;
        }

        let mut queue = VecDeque::new();
        let mut visited = HashMap::new();

        queue.push_back((s.to_string(), vec![s.to_string()]));

        let mut ret = Vec::new();

        while let Some((curr, path)) = queue.pop_front() {
            if curr == e {
                if ret.is_empty() || path.len() < ret.len() {
                    ret = path;
                    continue;
                }
            }

            if let Some(neighbors) = adj_list.get(&curr) {
                for neighbor in neighbors {
                    if !visited.contains_key(neighbor) {
                        let mut path_new = path.clone();
                        path_new.push(neighbor.clone());

                        queue.push_back((neighbor.clone(), path_new));
                        visited.insert(neighbor.clone(), true);
                    }
                }
            }
        }

        if ret.is_empty() {
            writeln!(out, "No solution.").unwrap();
        } else {
            for word in ret {
                writeln!(out, "{word}").unwrap();
            }
        }

        t += 1;
    }
}
