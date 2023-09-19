use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let chunks = input.chunks(worker_count);
    let mut result:HashMap<char,usize> = HashMap::new();

    let mut handles = vec![];
    for chunk in chunks {
        let chunk_strings = chunk.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        let handle = thread::spawn(move || {
            let mut map:HashMap<char,usize> = HashMap::new();
            for line in chunk_strings {
                for c in line.chars().filter(|c| c.is_alphabetic()) {
                    *map.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
                }
            }
            map
        });
        handles.push(handle);
    }

    for handle in handles {
        let map = handle.join().unwrap();
        for (k,v) in map {
            *result.entry(k).or_insert(0) += v;
        }
    }

    result
}
