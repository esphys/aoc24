use regex::Regex;
use std::str::Lines;

pub fn solution(input: Lines<'_>) {

    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();

    let test: Vec<usize> = input.clone().map(|s| {
        re.captures_iter(s).map(|caps| {
            let a = caps.name("a").unwrap().as_str().parse::<usize>().unwrap();
            let b = caps.name("b").unwrap().as_str().parse::<usize>().unwrap();
            a*b
        }).sum()

    }).collect();

    let silver: usize = test.iter().sum();

    let op_re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\)").unwrap();
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    
    let mut run = true;
    let gold: Vec<usize> = input.map(|s| {
        let instructions: Vec<&str> = op_re.find_iter(s).map(|m| {
            m.as_str()
        }).collect();
        let mut sum = 0;
        instructions.iter().for_each(|s| {
            match *s {
                "don't()" => run = false,
                "do()" => run = true,
                _ => {
                    let product: usize = mul_re.captures_iter(s).map(|caps| {
                        let (_, [a, b]) = caps.extract(); 
                        let ai = a.parse::<usize>().unwrap();
                        let bi = b.parse::<usize>().unwrap();

                        if run {
                            ai*bi 
                        } else {
                            0
                        }
                    }).sum();
                    sum += product;
                }
            }
        });
        sum
    }).collect();
    let gold_sum: usize = gold.iter().sum();
    println!("silver: {}", silver);
    println!("gold: {:?}", gold_sum);
}

