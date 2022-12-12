use std::cmp::Ordering;
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let mut totalCalories : u32 = 0;
        for line in lines {
            if let Ok(line_data) = line {
                if line_data.len() == 0 {
                    if totalCalories > answer {
                        answer = totalCalories;
                    }
                    totalCalories = 0;
                    continue;
                }

                let calories = line_data.parse::<u32>().unwrap();
                totalCalories += calories;
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let mut totalCalories : u32 = 0;
        let mut elvesCalories : Vec<u32> = vec![];
        for line in lines {
            if let Ok(line_data) = line {
                if line_data.len() == 0 {
                    elvesCalories.push(totalCalories);
                    totalCalories = 0;
                    continue;
                }

                let calories = line_data.parse::<u32>().unwrap();
                totalCalories += calories;
            }
        }
        elvesCalories.sort_by(|a, b| b.partial_cmp(a).unwrap());
        answer = elvesCalories[..3].iter().sum();
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}