use std::collections::HashSet;
use std::iter::FromIterator;
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
                let chars = line_data.chars().collect::<Vec<char>>();
                let left = HashSet::<char>::from_iter(chars[..chars.len() / 2].iter().cloned());
                let right = HashSet::<char>::from_iter(chars[chars.len() / 2..].iter().cloned());
                let intersect = left.intersection(&right).cloned().collect::<Vec<char>>();
                let char = intersect.first().cloned().unwrap() as u8;
                let priority = if char >= 'a' as u8 {char - 'a' as u8 + 1} else {char - 'A' as u8 + 27};
                answer += priority as u32;
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let lines2 = lines.collect::<Vec<Result<String, std::io::Error>>>();
        let groups = lines2.chunks(3);
        for group in groups {
            let line1 = group.get(0).unwrap().as_ref().unwrap();
            let line2 = group.get(1).unwrap().as_ref().unwrap();
            let line3 = group.get(2).unwrap().as_ref().unwrap();

            let line_data_1 = HashSet::<char>::from_iter(line1.chars().into_iter());
            let line_data_2 = HashSet::<char>::from_iter(line2.chars().into_iter());
            let line_data_3 = HashSet::<char>::from_iter(line3.chars().into_iter());

            let intersect1 = HashSet::<char>::from_iter(line_data_1.intersection(&line_data_2).cloned().collect::<Vec<char>>());
            let intersect2 = intersect1.intersection(&line_data_3).cloned().collect::<Vec<char>>();

            let char = intersect2.first().cloned().unwrap() as u8;
            let priority = if char >= 'a' as u8 {char - 'a' as u8 + 1} else {char - 'A' as u8 + 27};
            answer += priority as u32;
        }
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}