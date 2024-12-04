use std::str::Lines;
use std::collections::HashSet;

pub fn solution(input: Lines<'_>) {
    let mut silver = 0;
    let mut gold = 0;

    let mut input = input.peekable();
    let width = input.peek().map_or(0, |line| line.len());
    let height = input.clone().count();
    let mut grid: Vec<char> = Vec::with_capacity(width*height);
    
    let mut starts: Vec<usize> = Vec::with_capacity(width*height/3);
    let mut m_starts: Vec<usize> = Vec::with_capacity(width*height/3);

    let mut index = 0;
    input.for_each(|line| {
        line.chars().for_each(|c| {
            grid.push(c);
            match c {
                'X' => starts.push(index),
                'M' => m_starts.push(index),
                _ => {}
            }
            index += 1;
        });
    });
    
    grid.iter().enumerate().for_each(|(i, e)| {
        if *e == 'X' {
            starts.push(i);
        }
        if *e == 'M' {
            m_starts.push(i);
        }
    });

  println!("width: {}, height: {}", width, height);
      let mut x = 0;
      let mut y = 0;

    starts.iter().for_each(|&index| {
//        get x and y
        x = index % width;
        y = index / width;

        let needle = "XMAS".chars().collect::<Vec<char>>();
        // check forward
        if x <= width-4 {
            //check right
            if &grid[index..index+4] ==  needle {
                silver += 1;
            }
            // check right-up
            if y >= 3 {
//                if vec![grid[index], grid[index - width +1], grid[index- (2*width) + 2], grid[index- (3*width) + 3]] == needle {
                if needle[1]==grid[index-width+1] && needle[2]==grid[index-(2*width)+2] && needle[3]==grid[index-(3*width)+3] {
                    silver += 1;
                }
            }
            // check right-down
            if y <= height-4 {
//                if vec![grid[index], grid[index + width +1], grid[index + (2*width) + 2], grid[index + (3*width) + 3]] == needle {
                if needle[1]==grid[index+width+1] && needle[2]==grid[index+(2*width)+2] && needle[3]==grid[index+(3*width)+3] {
                    silver += 1;
                }
            }
        }
        // check down
        if y <= height-4 {
//            if vec![grid[index], grid[index + width], grid[index + (2*width)], grid[index + (3*width)]] == needle {
            if needle[1]==grid[index+width] && needle[2]==grid[index+(2*width)] && needle[3]==grid[index+(3*width)] {
                silver += 1;
            }           
        }
        // check up
        if y >= 3 {
//            if vec![grid[index], grid[index - width], grid[index - (2*width)], grid[index - (3*width)]] == needle {
            if needle[1]==grid[index-width] && needle[2]==grid[index-(2*width)] && needle[3]==grid[index-(3*width)] {
//                println!("found up =====> ({}, {})", x, y);
                silver += 1;
            }           
        }
        // check backwards
        if x >= 3 {
            //check left
//            if vec![grid[index], grid[index-1], grid[index-2], grid[index-3]] ==  needle {
            if needle[1]==grid[index-1] && needle[2]==grid[index-2] && needle[3]==grid[index-3] {
//                println!("found left =====> ({}, {})", x, y);
                silver += 1;
            }
            // check left-down
            if y >= 3 {
//                if vec![grid[index], grid[index - width - 1], grid[index- (2*width) - 2], grid[index- (3*width) - 3]] == needle {
                if needle[1]==grid[index-width-1] && needle[2]==grid[index-(2*width)-2] && needle[3]==grid[index-(3*width)-3] {
//                println!("found left-down =====> ({}, {})", x, y);
                    silver += 1;
                }
            }
            // check left-down
            if y <= height-4 {
//                if vec![grid[index], grid[index + width - 1], grid[index + (2*width) - 2], grid[index + (3*width) - 3]] == needle {
                if needle[1]==grid[index+width-1] && needle[2]==grid[index+(2*width)-2] && needle[3]==grid[index+(3*width)-3] {
//                println!("found left-up =====> ({}, {})", x, y);
                    silver += 1;
                }
            }
        }

    });

  println!("silver: {}", silver);

    let mut common_a: HashSet<usize> = HashSet::new();
    m_starts.iter().for_each(|&index| {
        x = index % width;
        y = index / width;

        let mas_needle = "MAS".chars().collect::<Vec<char>>();
        // check forward
        if x <= width-3 {
            // check right-up
            if y >= 2 {
//                if vec![grid[index], grid[index - width +1], grid[index- (2*width) + 2]] == mas_needle {
                if mas_needle[1]==grid[index-width+1] && mas_needle[2]==grid[index-(2*width)+2] {
                    if common_a.contains(&(index-width+1)) {
                        gold += 1;
                    } else {
                        common_a.insert(index-width+1);
                    }
                }
            }
            // check right-down
            if y <= height-3 {
//                if vec![grid[index], grid[index + width +1], grid[index + (2*width) + 2]] == mas_needle {
                if mas_needle[1]==grid[index+width+1] && mas_needle[2]==grid[index+(2*width)+2] {
                    if common_a.contains(&(index+width+1)) {
                        gold += 1;
                    } else {
                        common_a.insert(index+width+1);
                    }                   
                }
            }
        }
        // check backwards
        if x >= 2 {
            if y >= 2 {
//                if vec![grid[index], grid[index - width - 1], grid[index- (2*width) - 2]] == mas_needle {
                if mas_needle[1]==grid[index-width-1] && mas_needle[2]==grid[index-(2*width)-2] {
//                println!("found left-down =====> ({}, {})", x, y);
                    if common_a.contains(&(index-width-1)) {
                        gold += 1;
                    } else {
                        common_a.insert(index-width-1);
                    }        
                }
            }
            // check left-down
            if y <= height-3 {
//                if vec![grid[index], grid[index + width - 1], grid[index + (2*width) - 2]] == mas_needle {
                if mas_needle[1]==grid[index+width-1] && mas_needle[2]==grid[index+(2*width)-2] {
//                println!("found left-up =====> ({}, {})", x, y);
                    if common_a.contains(&(index+width-1)) {
                        gold += 1;
                    } else {
                        common_a.insert(index+width-1);
                    }        
                }
            }
        }
    });

    // find all the MAS diagonals, and note their A location
    
    // check the list of A locations for duplicates - if a duplicate exists, it's a cross

    println!("gold: {}", gold);
}
