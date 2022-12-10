use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let data = lines.into_iter().map(|n| n.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        for y in 1..data.len() - 1 {
            for x in 1..data[y].len() - 1 {
                let tree = data[y][x] as u8 - '0' as u8;

                let mut visible = false;
                for x_index in 0..x
                {
                    visible = true;
                    let comparer = data[y][x_index] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        visible = false;
                        break;
                    }
                }
                if visible
                {
                    answer += 1;
                    continue;
                }
                for x_index in x + 1..data[y].len()
                {
                    visible = true;
                    let comparer = data[y][x_index] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        visible = false;
                        break;
                    }
                }
                if visible
                {
                    answer += 1;
                    continue;
                }
                for y_index in 0..y
                {
                    visible = true;
                    let comparer = data[y_index][x] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        visible = false;
                        break;
                    }
                }
                if visible
                {
                    answer += 1;
                    continue;
                }
                for y_index in y + 1..data.len()
                {
                    visible = true;
                    let comparer = data[y_index][x] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        visible = false;
                        break;
                    }
                }
                if visible
                {
                    answer += 1;
                }
            }
        }

        answer += data.len() as u32 * 2;
        answer += data[0].len() as u32 * 2 - 4;

        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : u32 = 0;
        let data = lines.into_iter().map(|n| n.unwrap().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        for y in 1..data.len() - 1 {
            for x in 1..data[y].len() - 1 {
                let tree = data[y][x] as u8 - '0' as u8;

                let mut left_trees = 0;
                let mut right_trees = 0;
                let mut top_trees = 0;
                let mut bottom_trees = 0;

                for x_index in 0..x
                {
                    left_trees += 1;
                    let comparer = data[y][x - 1 - x_index] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        break;
                    }
                }
                for x_index in x + 1..data[y].len()
                {
                    right_trees += 1;
                    let comparer = data[y][x_index] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        break;
                    }
                }
                for y_index in 0..y
                {
                    top_trees += 1;
                    let comparer = data[y - 1 - y_index][x] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        break;
                    }
                }
                for y_index in y + 1..data.len()
                {
                    bottom_trees += 1;
                    let comparer = data[y_index][x] as u8 - '0' as u8;
                    if comparer >= tree
                    {
                        break;
                    }
                }

                let scenic_score = left_trees * right_trees * top_trees * bottom_trees;
                if scenic_score > answer {
                    answer = scenic_score;
                }
            }
        }

        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}