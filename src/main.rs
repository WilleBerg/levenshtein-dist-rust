use std::cmp::min;
use std::io;

fn main() {
    let cities: Vec<String> = std::fs::read_to_string("./places.txt").unwrap().lines().map(|s| s.to_string()).collect();
    loop {
        println!("Enter search term");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let cleaned_input = input.trim().to_string();
        if cleaned_input.to_lowercase() == "exit".to_string() {
            std::process::exit(0);
        }

        let mut result: Vec<(String, u32)> = vec![];
        for city in &cities {
            let lev_dist = lev_dist_v2(&city, &cleaned_input);
            if lev_dist < 5 {
                result.push((city.clone(), lev_dist));
            }
        }
        result.sort_by_key(|e| e.1);
        let mut counter = 0;
        for res in result {
            let full: u32 = cleaned_input.len() as u32;
            let match_prc: f32 = (full - res.1) as f32 / full as f32;
            println!("{}, {:.2}", res.0, match_prc * 100.0);
            if counter == 10 {
                break;
            }
            counter += 1;
        }
    }
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
        for j in 0..t.len() { // n - 1
            let del_cost = v0.get(j + 1).unwrap() + 1;
            let ins_cost = v1.get(j).unwrap() + 1;
            let sub_cost: u32;
            if s.chars().nth(i as usize).unwrap() == t.chars().nth(j).unwrap() {
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
