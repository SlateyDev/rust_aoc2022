use std::collections::LinkedList;
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut towers: Vec<LinkedList<char>> = vec![
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new()
        ];
        let mut towers_filled = false;

        for line in lines {
            if let Ok(line_data) = line {
                if !towers_filled {
                    let line_chars = line_data.chars().collect::<Vec<char>>();
                    if line_data.len() == 0 {
                        towers_filled = true;
                        //remove last line of tower
                        for tower_index in 0..9 {
                            towers[tower_index].pop_front();
                        }
                        continue;
                    }

                    //fill the tower
                    for tower_index in 0..9 {
                        let char = line_chars[tower_index * 4 + 1];
                        if char != ' ' {towers[tower_index].push_front(char);}
                    }
                    continue;
                }

                //move items
                let action_split : Vec<u32> = line_data
                    .replace("move ", "")
                    .replace("from ", "")
                    .replace("to ", "")
                    .split(" ")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                let move_amount = action_split[0];
                let from : usize = action_split[1] as usize - 1;
                let to : usize = action_split[2] as usize - 1;

                for _ in 0..move_amount {
                    let popped = towers[from].pop_back().unwrap();
                    towers[to].push_back(popped);
                }
            }
        }

        let mut answer: String = String::from("");
        for mut tower in towers {
            let top = tower.back().unwrap();
            answer += top.to_string().as_str();
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut towers: Vec<LinkedList<char>> = vec![
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new(),
            LinkedList::new()
        ];
        let mut towers_filled = false;

        for line in lines {
            if let Ok(line_data) = line {
                if !towers_filled {
                    let line_chars = line_data.chars().collect::<Vec<char>>();
                    if line_data.len() == 0 {
                        towers_filled = true;
                        //remove last line of tower
                        for tower_index in 0..9 {
                            towers[tower_index].pop_front();
                        }
                        continue;
                    }

                    //fill the tower
                    for tower_index in 0..9 {
                        let char = line_chars[tower_index * 4 + 1];
                        if char != ' ' {towers[tower_index].push_front(char);}
                    }
                    continue;
                }

                //move items
                let action_split : Vec<u32> = line_data
                    .replace("move ", "")
                    .replace("from ", "")
                    .replace("to ", "")
                    .split(" ")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect();

                let move_amount = action_split[0];
                let from : usize = action_split[1] as usize - 1;
                let to : usize = action_split[2] as usize - 1;

                let from_len = towers[from].len();
                let mut popped = towers[from].split_off(from_len - move_amount as usize);
                towers[to].append(&mut popped);
            }
        }

        let mut answer: String = String::from("");
        for mut tower in towers {
            let top = tower.back().unwrap();
            answer += top.to_string().as_str();
        }
        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}