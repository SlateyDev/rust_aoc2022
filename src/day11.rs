use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

struct Monkey {
    items: Vec<i64>,
    operation: fn (i64, String) -> i64,
    operation_value: String,
    test_divisor: i64,
    true_monkey: u32,
    false_monkey: u32,

    inspections: i64,
}

impl Monkey {
    fn add(old: i64, val: String) -> i64 {old + Monkey::get_operation_value(old, val)}
    fn mul(old: i64, val: String) -> i64 {old * Monkey::get_operation_value(old, val)}
    fn get_operation_value(old: i64, val: String) -> i64 {
        if val == "old" {old} else {val.parse::<i64>().unwrap()}
    }
}

fn load_monkeys(lines: Lines<BufReader<File>>, monkeys: &mut Vec<Monkey>) {
    let line_map: Vec<String> = lines.map(|l| l.unwrap()).collect();
    let groups = line_map.chunks(7);

    let mut items: Vec<i64> = vec![];
    let mut operation_type = String::from("");
    let mut operation_value = String::from("");
    let mut test_divisor: i64 = 0;
    let mut true_monkey: u32 = 0;
    let mut false_monkey: u32 = 0;

    for group in groups {
        items = group[1].trim().replace("Starting items: ", "").split(", ").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        let line_split = group[2].trim().replace("Operation: ", "").split(" ").map(|n| String::from(n)).collect::<Vec<String>>();
        operation_type = line_split[3].clone();
        operation_value = line_split[4].clone();

        test_divisor = group[3].trim().replace("Test: divisible by ", "").parse::<i64>().unwrap();

        true_monkey = group[4].trim().replace("If true: throw to monkey ", "").parse::<u32>().unwrap();

        false_monkey = group[5].trim().replace("If false: throw to monkey ", "").parse::<u32>().unwrap();

        monkeys.push(Monkey {
            items: items.clone(),
            operation: if operation_type == "*" { Monkey::mul } else { Monkey::add },
            operation_value: operation_value.clone(),
            test_divisor,
            true_monkey,
            false_monkey,
            inspections: 0
        });
    }
}

fn simulate(monkeys: &mut Vec<Monkey>, rounds: usize, func: impl Fn(i64) -> i64) {
    for _ in 0..rounds {
        for mut monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];

            let items : Vec<i64> = monkey.items.drain(..).collect();
            monkey.inspections += items.len() as i64;

            let operation = monkey.operation;
            let operation_value = monkey.operation_value.clone();
            let test_divisor = monkey.test_divisor;
            let true_monkey = monkey.true_monkey;
            let false_monkey = monkey.false_monkey;

            for item in items {
                let mut inspection_item = item;

                inspection_item = (operation)(inspection_item, operation_value.clone());
                inspection_item = func(inspection_item);

                let target = if inspection_item % test_divisor == 0 {
                    true_monkey
                } else {
                    false_monkey
                } as usize;
                monkeys[target].items.push(inspection_item)
            }
        }
    }
}

fn part1() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut monkeys: Vec<Monkey> = vec![];
        load_monkeys(lines, &mut monkeys);

        simulate(&mut monkeys, 20, |x| x / 3);

        let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<i64>>();
        inspections.sort_unstable();
        let answer : i64 = inspections.iter().rev().take(2).product();

        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

fn part2() {
    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut monkeys: Vec<Monkey> = vec![];
        load_monkeys(lines, &mut monkeys);

        let modulo : i64 = monkeys.iter().map(|x| x.test_divisor).product();

        simulate(&mut monkeys, 10000, |x| x % modulo);

        let mut inspections = monkeys.iter().map(|m| m.inspections).collect::<Vec<i64>>();
        inspections.sort_unstable();
        let answer : i64 = inspections.iter().rev().take(2).product();

        println!("Day {} part 2: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}