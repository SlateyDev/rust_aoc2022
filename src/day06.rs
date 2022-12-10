use std::collections::HashSet;
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer: u32 = 0;
        let marker_size = 4;
        let chars = lines.into_iter().map(|n| n.unwrap()).collect::<Vec<String>>()[0].chars().collect::<Vec<char>>();
        for char_index in 0..chars.len() - marker_size {
            let slice = &chars[char_index..char_index + marker_size];
            let hs = HashSet::<char>::from_iter(slice.iter().cloned());
            if hs.len() == marker_size {
                answer = (char_index + marker_size) as u32;  //Add the 4 characters since our index doesn't take that into account
                break;
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer: u32 = 0;
        let marker_size = 14;
        let chars = lines.into_iter().map(|n| n.unwrap()).collect::<Vec<String>>()[0].chars().collect::<Vec<char>>();
        for char_index in 0..chars.len() - marker_size {
            let slice = &chars[char_index..char_index + marker_size];
            let hs = HashSet::<char>::from_iter(slice.iter().cloned());
            if hs.len() == marker_size {
                answer = (char_index + marker_size) as u32;  //Add the 4 characters since our index doesn't take that into account
                break;
            }
        }
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}