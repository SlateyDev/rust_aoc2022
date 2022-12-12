use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let mut total_calories: u32 = 0;
        for line in lines {
            if let Ok(line_data) = line {
                if line_data.len() == 0 {
                    if total_calories > answer {
                        answer = total_calories;
                    }
                    total_calories = 0;
                    continue;
                }

                let calories = line_data.parse::<u32>().unwrap();
                total_calories += calories;
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let mut total_calories: u32 = 0;
        let mut elves_calories: Vec<u32> = vec![];
        for line in lines {
            if let Ok(line_data) = line {
                if line_data.len() == 0 {
                    elves_calories.push(total_calories);
                    total_calories = 0;
                    continue;
                }

                let calories = line_data.parse::<u32>().unwrap();
                total_calories += calories;
            }
        }
        elves_calories.sort_by(|a, b| b.partial_cmp(a).unwrap());
        answer = elves_calories[..3].iter().sum();
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}