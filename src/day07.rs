use std::collections::HashMap;
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let mut current_dir : String = String::from("");
        let mut folders = HashMap::<String, u32>::new();
        for line in lines {
            if let Ok(line_data) = line {
                let line_split = line_data.split(" ").into_iter().collect::<Vec<&str>>();

                match line_split[0] {
                    "dir" => continue,
                    "$" => {
                        match line_split[1] {
                            "cd" => {
                                match line_split[2] {
                                    "/" => {
                                        current_dir = String::from("");
                                    },
                                    ".." => {
                                        current_dir = current_dir.trim_end_matches(|n| n != '/').to_string();
                                        current_dir = current_dir.trim_end_matches(|n| n == '/').to_string();
                                    },
                                    _ => {
                                        if current_dir.len() > 0 {current_dir += "/";}
                                        current_dir += line_split[2];
                                    }
                                }
                            },
                            _ => continue,
                        }
                    },
                    _ => {
                        *folders.entry(current_dir.clone()).or_insert(0) += line_split[0].parse::<u32>().unwrap();

                        let mut temp_dir = current_dir.clone();

                        while temp_dir != "" {
                            temp_dir = temp_dir.trim_end_matches(|n| n != '/').to_string();
                            temp_dir = temp_dir.trim_end_matches(|n| n == '/').to_string();
                            *folders.entry(temp_dir.clone()).or_insert(0) += line_split[0].parse::<u32>().unwrap();
                        }
                    },
                }
            }
        }

        for folder in folders {
            if folder.1 <= 100000 {
                answer += folder.1;
            }
        }

        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut current_dir : String = String::from("");
        let mut folders = HashMap::<String, u32>::new();
        for line in lines {
            if let Ok(line_data) = line {
                let line_split = line_data.split(" ").into_iter().collect::<Vec<&str>>();

                match line_split[0] {
                    "dir" => continue,
                    "$" => {
                        match line_split[1] {
                            "cd" => {
                                match line_split[2] {
                                    "/" => {
                                        current_dir = String::from("");
                                    },
                                    ".." => {
                                        current_dir = current_dir.trim_end_matches(|n| n != '/').to_string();
                                        current_dir = current_dir.trim_end_matches(|n| n == '/').to_string();
                                    },
                                    _ => {
                                        if current_dir.len() > 0 {current_dir += "/";}
                                        current_dir += line_split[2];
                                    }
                                }
                            },
                            _ => continue,
                        }
                    },
                    _ => {
                        *folders.entry(current_dir.clone()).or_insert(0) += line_split[0].parse::<u32>().unwrap();

                        let mut temp_dir = current_dir.clone();

                        while temp_dir != "" {
                            temp_dir = temp_dir.trim_end_matches(|n| n != '/').to_string();
                            temp_dir = temp_dir.trim_end_matches(|n| n == '/').to_string();
                            *folders.entry(temp_dir.clone()).or_insert(0) += line_split[0].parse::<u32>().unwrap();
                        }
                    },
                }
            }
        }

        let space_available = 70_000_000;
        let unused_space_needed = 30_000_000;
        let space_taken = folders[""];
        let current_unused_space = space_available - space_taken;

        let mut smallest_acceptable_folder = space_available;

        for folder in folders {
            let this_folder_makes_available = current_unused_space + folder.1;
            if this_folder_makes_available >= unused_space_needed && folder.1 < smallest_acceptable_folder {
                smallest_acceptable_folder = folder.1;
            }
        }

        let mut answer : u32 = smallest_acceptable_folder;

        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}