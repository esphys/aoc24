use std::str::Lines;
pub fn solution(input: Lines<'_>) {

    let reports: Vec<Vec<usize>> = input.map(|line| line.split(" ")
        .map(|s| s.parse::<usize>().unwrap()).collect()).collect();

    let mut silver = 0;
    let mut gold = 0;

    reports.iter().for_each(|report| {
        let mut safe = 0;
        if report[0] == report[1]  { safe = 1; } 
        let increasing = report[0] < report[1];
        for i in 1..report.len() {
            // check inc/dec consistency
            if (report[i-1] < report[i]) != increasing {
                safe += 1;
                continue;
            }
            // check length
            let diff = report[i-1].abs_diff(report[i]);
            if diff > 3 || diff < 1 {
               safe += 1;
               continue;
            }
        }

        let mut safe2 = false;
        if safe != 0 {
            for i in 0..report.len() {
                let new_report: Vec<&usize> = report.iter().enumerate()
                    .filter(|(j, _)| j != &i)
                    .map(|(_, e)| e).collect();
                let increasing2 = new_report[0] < new_report[1]; 
                let valid: usize = new_report.windows(2).map(|a| {
                    if (a[0].abs_diff(*a[1]) < 1 || a[0].abs_diff(*a[1]) > 3)
                        || ((a[0] < a[1]) != increasing2) {
                        return 1 
                    }
                    0
                }).sum();

                if valid == 0 {
                    // this means there was a safe array
                    safe2 = true;
                }
                
            }
        }

        if safe == 0 { silver += 1 };
        if safe == 0 || safe2 { gold += 1 };
        
    });

    println!("silver: {}", silver);
    println!("gold: {}", gold);
}

