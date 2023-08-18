use std::cmp::min;
use std::collections::HashMap;
use std::env::args;
use std::sync::{Arc, Mutex};
use std::thread;

const MAX_NGRAMS: u32 = 1_000;
const PRINT_DEBUG: bool = true;

const NGRAM_WEIGHT: u32 = 3;
const FILENAME_WEIGHT: u32 = 6;

const NGRAM_SIZE: u32 = 3;
const MAX_PRINTS: usize = 25;

fn main() {
    let args = args().collect::<Vec<String>>();
    let input = &args[1];
    // run_ngram_approach(input);
    run_ngram_approach_v2(input);
}

fn run_ngram_approach_v2(input: &String) {
    let print_verbose = if PRINT_DEBUG {
        |s: &str| {
            println!("{}", s);
        }
    } else {
        |_s: &str| {}
    };

    let mut result: Vec<(&String, i32)> = vec![];
    let mut amount_matching_ngrams: HashMap<String, u32> = HashMap::new();

    print_verbose("Reading cache file");
    let cache_line: Vec<String> = std::fs::read_to_string("./cache.txt")
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();

    let mut current_line: u32 = 0;

    // let input_ngrams = generate_ngram(3, &input);
    let input_ngrams = generate_ngram_bytes(NGRAM_SIZE, &input);

    print_verbose("Beginning ngram creation");
    loop {
        print_verbose(format!("Size of hashmap: {}", amount_matching_ngrams.len()).as_str());
        let mut tmp: Vec<&String> = vec![];
        loop {
            tmp.push(if let Some(val) = cache_line.get(current_line as usize) {
                &val
            } else {
                current_line += 1;
                continue;
            });
            current_line += 1;
            if current_line % MAX_NGRAMS == 0 || current_line >= cache_line.len() as u32 {
                break;
            }
        }
        print_verbose("Generating ngrams");
        // let data_ngram = generate_ngrams(3, &tmp);
        let data_ngram = generate_ngrams_bytes(NGRAM_SIZE, &tmp);
        print_verbose("Done");
        for ngram in &input_ngrams {
            if let Some(val) = data_ngram.get(ngram) {
                for entry in val {
                    let e = amount_matching_ngrams.entry((*entry).clone()).or_insert(0);
                    *e += 1;
                }
            }
        }
        if current_line >= cache_line.len() as u32 {
            break;
        }
    }

    let sort_key_val: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let handle = thread::spawn({
        let amount_matching_ngrams = amount_matching_ngrams.clone();
        let sort_key_val = sort_key_val.clone();
        let input = input.clone();
        move || {
            for (k, _) in &amount_matching_ngrams {
                let lev_dist = filename_lev_distance(&k, &input);
                let mut map = sort_key_val.lock().unwrap(); // Get mutex lock
                map.insert(k.clone(), (lev_dist * FILENAME_WEIGHT) as i32 - (*amount_matching_ngrams.get(k).unwrap_or(&0) * NGRAM_WEIGHT) as i32);
            }
        }
    });

    print_verbose("Done creating ngrams, now filling result");
    for (k, _) in &amount_matching_ngrams {
        let lev_dist = lev_dist_v2(&k, &input);
        result.push((&k, lev_dist as i32));
    }
    print_verbose("Result fill done");
    handle.join().unwrap();
    print_verbose("Thread done");
    print_verbose("Now sorting result");
    result.sort_by_key(|e| {
        e.1 + sort_key_val.lock().unwrap().get(e.0).unwrap()
        // e.1 - (*amount_matching_ngrams.get(e.0).unwrap_or(&0) * NGRAM_WEIGHT) as i32 + (filename_lev_distance(&e.0, input) * FILENAME_WEIGHT) as i32
    });

    let mut c = 0;
    println!("Top results:");
    for res in &result {
        println!(
            "{}, {}",
            res.0,
            res.1 + sort_key_val.lock().unwrap().get(res.0).unwrap()
            // res.1 - (*amount_matching_ngrams.get(res.0).unwrap_or(&0) * NGRAM_WEIGHT) as i32 + (filename_lev_distance(&res.0, input) * FILENAME_WEIGHT) as i32
        );
        c += 1;
        if c == MAX_PRINTS{
            println!("...\n+ {} results.", result.len() - MAX_PRINTS);
            break;
        }
    }
}

fn filename_lev_distance(full_path: &String, search_term: &String) -> u32 {
    let path_split = full_path.split('/').collect::<Vec<&str>>();
    let file_name = path_split.get(path_split.len() - 1).unwrap();
    lev_dist_v2(&file_name.to_string(), search_term)
}

fn generate_ngrams<'a>(size: u32, vec: &Vec<&'a String>) -> HashMap<String, Vec<&'a String>> {
    let mut hmap: HashMap<String, Vec<&String>> = HashMap::new();
    for entry in vec {
        // println!("{}", entry);
        if *entry == "" {
            continue;
        }
        for i in 0..(entry.len() - size as usize) {
            let ngram = match entry.get((0 + i)..(size as usize + i)) {
                Some(val) => val.to_string(),
                None => {
                    eprintln!("error creating ngram for {}", entry);
                    continue;
                }
            };
            let e = hmap.entry(ngram).or_insert(vec![]);
            e.push(entry);
        }
    }
    hmap
}

fn generate_ngrams_bytes<'a>(
    size: u32,
    vec: &Vec<&'a String>,
) -> HashMap<Vec<u8>, Vec<&'a String>> {
    let mut hmap: HashMap<Vec<u8>, Vec<&String>> = HashMap::new();
    for entry in vec {
        if *entry == "" {
            continue;
        }
        let size_usize = size as usize;
        for i in 0..(entry.len() - size_usize) {
            let ngram = entry.as_bytes().get(i..(i + size_usize)).unwrap();
            let e = hmap
                .entry(ngram.iter().map(|b| b.to_owned()).collect::<Vec<u8>>())
                .or_insert(vec![]);
            e.push(entry);
        }
    }
    hmap
}

fn generate_ngram(size: u32, word: &String) -> Vec<String> {
    let mut rvec: Vec<String> = vec![];
    for i in 0..(word.len() - size as usize) {
        let ngram = match word.get((0 + i)..(size as usize + i)) {
            Some(val) => val.to_string(),
            None => continue,
        };
        rvec.push(ngram);
    }
    rvec
}

fn generate_ngram_bytes(size: u32, word: &String) -> Vec<Vec<u8>> {
    let mut rvec: Vec<Vec<u8>> = vec![];
    let size_usize = size as usize;
    for i in 0..(word.len() - size_usize) {
        let ngram = word.as_bytes().get(i..(i + size_usize)).unwrap();
        rvec.push(ngram.iter().map(|b| b.to_owned()).collect());
    }
    rvec
}

fn _lev_dist(s1: &String, s2: &String) -> usize {
    let mut r_matrix: Vec<Vec<usize>> = vec![vec![0; s1.len() + 1]; s2.len() + 1];

    for y in 0..r_matrix.len() {
        r_matrix[y][0] = y;
    }
    for x in 0..r_matrix[0].len() {
        r_matrix[0][x] = x;
    }

    for j in 1..r_matrix[0].len() {
        for i in 1..r_matrix.len() {
            let s_cost: usize;
            if s1.chars().nth(j - 1) == s2.chars().nth(i - 1) {
                s_cost = 0;
            } else {
                s_cost = 1;
            }

            r_matrix[i][j] = min(
                r_matrix[i - 1][j] + 1,
                min(r_matrix[i][j - 1] + 1, r_matrix[i - 1][j - 1] + s_cost),
            );
        }
    }

    r_matrix[s2.len()][s1.len()]
}

fn lev_dist_v2(s: &String, t: &String) -> u32 {
    let n = t.len() as u32;
    let m = s.len() as u32;

    let mut v0: Vec<u32> = vec![0; (n + 1) as usize];
    let mut v1: Vec<u32> = vec![0; (n + 1) as usize];

    let mut counter = 0;
    v0.iter_mut().for_each(|e| {
        *e = counter;
        counter += 1;
    });

    for i in 0..m {
        v1[0] = i + 1;

        let s_nth = match s.chars().nth(i as usize) {
            Some(char) => char,
            None => '\0',
        };

        for j in 0..t.len() {
            // n - 1
            let del_cost = v0.get(j + 1).unwrap() + 1;
            let ins_cost = v1.get(j).unwrap() + 1;
            let sub_cost: u32;
            let t_nth = match t.chars().nth(j) {
                Some(char) => char,
                None => '\0',
            };
            if s_nth == t_nth && s_nth != '\0' {
                sub_cost = *v0.get(j).unwrap();
            } else {
                sub_cost = *v0.get(j).unwrap() + 1;
            }
            v1[j + 1] = min(del_cost, min(ins_cost, sub_cost));
        }
        v0 = v1.clone();
    }
    v0[t.len()]
}
