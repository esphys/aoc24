use std::str::Lines;
use std::collections::HashMap;

pub fn solution(input: Lines<'_>) {
    let mut silver = 0;
    let mut gold = 0;

    let hash1: HashMap<usize, usize> = HashMap::new();


    let (mut group1, mut group2): (Vec<usize>, Vec<usize>) = input
        .map(|s| s.split_once("   ").unwrap())
        .map(|(a, b)| [a, b])
        .map(|ab| ab.map(|s| s.parse::<usize>().unwrap()))
        .map(|[a, b]| (a, b)).unzip();

    group1.sort_unstable();
    group2.sort_unstable();

    group1.iter().for_each(|left| {
        if hash1.contains_key(left) {
            gold += left * hash1.get(left).unwrap();
        }
    });
    let mut buf: usize;
    
    for i in 0..group1.len() {
       buf = group1.get(i).unwrap().abs_diff(*group2.get(i).unwrap()); 
       silver += buf;
    }

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}

