struct Dial {
    position: u16,
}

impl Dial {
    fn new(start_position: u16) -> Self {
        Dial {
            position: start_position,
        }
    }

    fn step(&mut self, steps: u16, is_right: bool) {
        if is_right {
            self.position = (self.position + steps) % 100;
        } else {
            self.position = (self.position + 100 - steps % 100) % 100;
        }
    }

    fn step_sequence_0_count_end_position(&mut self, steps: &[(u16, bool)]) -> u16 {
        let mut count = 0;
        for (step, is_right) in steps {
            self.step(*step, *is_right);
            if self.position == 0 {
                count += 1;
            }
        }
        count
    }

    fn count_pass_0_with_steps(&self, steps: u16, is_right: bool) -> u16 {
        let full_cycles = steps / 100;
        let remaining_steps = steps % 100;

        if remaining_steps == 0
            || self.position == 0
            || (is_right && self.position + remaining_steps < 100)
            || (!is_right && self.position > remaining_steps)
        {
            return full_cycles;
        }
        return full_cycles + 1;
    }

    fn step_sequence_0_count_any_position(&mut self, steps_sequence: &[(u16, bool)]) -> u16 {
        let mut count = 0;
        for (step, is_right) in steps_sequence {
            count += self.count_pass_0_with_steps(*step, *is_right);
            self.step(*step, *is_right);
        }
        count
    }
}

fn main() {
    let input = read_txt_file("input.txt"); // Rust: make sure input.txt is in the same directory as main.rs
    let lines: Vec<&str> = input.lines().collect();

    let steps: Vec<(u16, bool)> = lines
        .iter()
        .map(|line| {
            let is_right = match &line[0..1] {
                "R" => true,
                "L" => false,
                _ => panic!("Invalid direction"),
            };
            let steps = line[1..].parse::<u16>().unwrap();
            (steps, is_right)
        })
        .collect();

    // let compressed_steps : Vec<(u16, bool)> = {
    //     let mut result : Vec<(u16, bool)> = Vec::new();
    //     for ( steps, is_right ) in steps {
    //         if let Some( last ) = result.last_mut() {
    //             if last.1 == is_right {
    //                 last.0 += steps;
    //                 continue;
    //             }
    //         }
    //         result.push( ( steps, is_right ) );
    //     }
    //     result
    // };

    let mut dial = Dial::new(50);
    let result = dial.step_sequence_0_count_end_position(&steps);
    println!("Result End: {}, Dial position: {}", result, dial.position);

    let mut dial = Dial::new(50);
    let result = dial.step_sequence_0_count_any_position(&steps);
    println!("Result Any: {}, Dial position: {}", result, dial.position);
}

fn read_txt_file(path: &str) -> String {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path).expect(&format!("Could not open file: {}", path));
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
