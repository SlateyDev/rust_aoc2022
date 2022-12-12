use std::collections::HashMap;
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let answer = get_tail_movements(lines.map(|n| n.unwrap()).collect(), 2);

        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let answer = get_tail_movements(lines.map(|n| n.unwrap()).collect(), 10);

        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn get_tail_movements(lines : Vec<String>, knot_length: usize) -> usize {
    let mut knots = vec![(0 as i32, 0 as i32); knot_length];
    let mut tail_positions = HashMap::<(i32, i32), usize>::new();

    tail_positions.insert((0, 0), 1);

    for line in lines {
        let data = line.split(" ").collect::<Vec<&str>>();

        let delta = match data[0] {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!(),
        };

        for _ in 0..data[1].parse::<usize>().unwrap() {
            knots[0] = (knots[0].0 + delta.0, knots[0].1 + delta.1);

            for knot_index in 1..knots.len() {
                if (knots[knot_index].0 - knots[knot_index - 1].0).abs() > 1 || (knots[knot_index].1 - knots[knot_index - 1].1).abs() > 1 {
                    if knots[knot_index - 1].0 != knots[knot_index].0 && knots[knot_index - 1].1 != knots[knot_index].1
                    {
                        knots[knot_index].0 += -(knots[knot_index].0 - knots[knot_index - 1].0).signum();
                        knots[knot_index].1 += -(knots[knot_index].1 - knots[knot_index - 1].1).signum();
                    }
                    else if (knots[knot_index].0 - knots[knot_index - 1].0).abs() > 1
                    {
                        knots[knot_index].0 += -(knots[knot_index].0 - knots[knot_index - 1].0).signum();
                    }
                    else if (knots[knot_index].1 - knots[knot_index - 1].1).abs() > 1
                    {
                        knots[knot_index].1 += -(knots[knot_index].1 - knots[knot_index - 1].1).signum();
                    }
                    if knot_index == knots.len() - 1
                    {
                        *tail_positions.entry(knots[knot_index]).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    tail_positions.len()
}