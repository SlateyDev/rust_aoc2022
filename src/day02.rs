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
                let line_split = line_data.split(" ").collect::<Vec<&str>>();
                let them = line_split[0].chars().next().unwrap() as u8 - 'A' as u8;
                let you = line_split[1].chars().next().unwrap() as u8 - 'X' as u8;

                answer += (you + 1) as u32;
                if you == them {answer += 3;}
                else if you == (them + 1) % 3 {answer += 6}
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
                let line_split = line_data.split(" ").collect::<Vec<&str>>();
                let them = line_split[0].chars().next().unwrap() as u8 - 'A' as u8;
                let result = line_split[1].chars().next().unwrap() as u8 - 'X' as u8;
                let you = (them + result + 2) % 3;
                answer += (you + 1 + result * 3) as u32;
            }
        }
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}