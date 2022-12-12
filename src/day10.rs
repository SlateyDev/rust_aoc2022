use crate::common_funcs::read_lines;

pub fn run() {
    part1();
    part2();
}

struct DeviceProcessor1 {
    cycles : i32,
    x_reg : i32,
    signal_strength : i32,
}

impl DeviceProcessor1 {
    fn new() -> DeviceProcessor1 {
        DeviceProcessor1 {
            cycles: 0,
            x_reg: 1,
            signal_strength: 0,
        }
    }
    pub fn noop(&mut self) {
        self.handle_cycle();
    }
    pub fn addx(&mut self, val: i32) {
        self.handle_cycle();
        self.handle_cycle();
        self.x_reg += val;
    }
    fn handle_cycle(&mut self) {
        self.cycles += 1;
        if self.cycles % 40 != 20 || self.cycles > 220 {return}
        self.signal_strength += self.x_reg * self.cycles;
    }
}

fn part1() {
    let mut processor = DeviceProcessor1::new();

    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : i32 = 0;
        for line in lines {
            if let Ok(line_data) = line {
                if line_data == "noop" {processor.noop()}
                else {processor.addx(line_data.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap())}
            }
        }
        answer = processor.signal_strength;
        println!("Day {} part 1: {}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}

struct DeviceProcessor2 {
    cycles : i32,
    x_reg : i32,
    lines : Vec::<String>,
}

impl DeviceProcessor2 {
    fn new() -> DeviceProcessor2 {
        DeviceProcessor2 {
            cycles: 0,
            x_reg: 1,
            lines: vec![
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from(""),
                String::from("")
            ],
        }
    }
    pub fn noop(&mut self) {
        self.handle_cycle();
    }
    pub fn addx(&mut self, val: i32) {
        self.handle_cycle();
        self.handle_cycle();
        self.x_reg += val;
    }
    fn handle_cycle(&mut self) {
        let col = self.cycles % 40;
        let row = self.cycles / 40;
        self.lines[row as usize] += if self.x_reg >= col - 1 && self.x_reg <= col + 1 {"#"} else {" "};
        self.cycles += 1;
    }
}

fn part2() {
    let mut processor = DeviceProcessor2::new();

    if let Ok(lines) = read_lines(format!("input-{}.txt", file!().replace("src\\", "").replace(".rs", ""))) {
        let mut answer : String;
        for line in lines {
            if let Ok(line_data) = line {
                if line_data == "noop" {processor.noop()}
                else {processor.addx(line_data.split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap())}
            }
        }
        answer = processor.lines.join("\n");
        println!("Day {} part 2: \n{}", file!().replace("src\\day", "").replace(".rs", ""), answer);
    }
}