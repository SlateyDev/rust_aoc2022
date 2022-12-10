use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        for line in lines {
            if let Ok(line_data) = line {
                let split: Vec<&str> = line_data.as_str().split(",").collect();
                let first: Vec<&str> = split.get(0).unwrap().split("-").collect();
                let second: Vec<&str> = split.get(1).unwrap().split("-").collect();

                let first1 = first[0].parse::<u32>().unwrap();
                let first2 = first[1].parse::<u32>().unwrap();
                let second1 = second[0].parse::<u32>().unwrap();
                let second2 = second[1].parse::<u32>().unwrap();

                if (first1 >= second1 && first2 <= second2) || (second1 >= first1 && second2 <= first2) {
                    answer += 1;
                }
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        for line in lines {
            if let Ok(line_data) = line {
                let split: Vec<&str> = line_data.as_str().split(",").collect();
                let first: Vec<&str> = split.get(0).unwrap().split("-").collect();
                let second: Vec<&str> = split.get(1).unwrap().split("-").collect();

                let first1 = first[0].parse::<u32>().unwrap();
                let first2 = first[1].parse::<u32>().unwrap();
                let second1 = second[0].parse::<u32>().unwrap();
                let second2 = second[1].parse::<u32>().unwrap();

                if !(first1 > second2 || second1 > first2) {
                    answer += 1;
                }
            }
        }
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}