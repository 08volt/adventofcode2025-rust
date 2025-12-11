use std::collections::{HashSet, VecDeque};
use std::fs;

fn read_txt_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open file: {}", path))
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Machine {
    level: usize,
}

impl Machine {
    fn new(level: usize) -> Machine {
        Machine {
            level,
        }
    }

    fn toggle(&self) -> Machine {
        Machine {
            level: self.level + 1,
        }
    }
}

#[derive(Debug, Hash)]
struct MachineSequence {
    machines: Vec<Machine>,
    goal: Vec<Machine>,
    buttons: Vec<Button>,
}

impl MachineSequence {
    fn new(goal: Vec<Machine>, buttons: Vec<Button>) -> MachineSequence {
        let machines = vec![Machine::new(0); goal.len()];
        MachineSequence {
            machines,
            goal,
            buttons,
        }
    }

    fn step_to_get_to_goal(&self) -> usize {
        let mut current_machines: HashSet<Vec<Machine>> = [self.machines.clone()].into_iter().collect();
        let mut count = 0;

        let mut past_machines: HashSet<Vec<Machine>> = HashSet::new();

        while !current_machines.contains(&self.goal) {
            past_machines.extend(current_machines.clone());

            let nexts: Vec<Vec<Machine>> = current_machines.iter().flat_map(
                |machines| self.buttons.iter().map(|button| button.press(machines.clone()))
            ).filter(|machines| !self.exceeds_joltage_req(machines) && !past_machines.contains(machines)).collect();

            current_machines = nexts.into_iter().collect();
            count += 1;
        }

        count
    }



    #[inline]
    fn exceeds_joltage_req(&self, machines: &[Machine]) -> bool {
        machines
            .iter()
            .zip(&self.goal)
            .any(|(m, g)| m.level > g.level)
    }


}

#[derive(Debug, Hash, Clone)]
struct Button {
    sequence: Vec<usize>,
}

impl Button {
    fn new(sequence: Vec<usize>) -> Button {
        Button { sequence }
    }

    fn press(&self, machine_sequence: Vec<Machine>) -> Vec<Machine> {
        let mut result = machine_sequence;
        for &idx in &self.sequence {
            if idx < result.len() {
                result[idx] = result[idx].toggle();
            }
        }
        result
    }
}

fn parse_line(line: &str) -> MachineSequence {
    let parts: Vec<&str> = line.split(|c| c == '[' || c == ']' || c == '{' || c == '}')
        .filter(|s| !s.trim().is_empty())
        .collect();

    // let goal_str = parts[0].trim();
    // let goal: Vec<Machine> = goal_str.chars()
    //     .filter(|&c| c == '.' || c == '#')
    //     .map(|c| Machine::new(c == '#'))
    //     .collect();

    let buttons_str = parts[1].trim();

    let buttons: Vec<Button> = buttons_str
        .split(')')
        .filter(|s| s.trim().starts_with('('))
        .map(|s| {
            let inner = s.trim().trim_start_matches('(').trim();
            let sequence: Vec<usize> = inner
                .split(',')
                .map(|n| n.trim().parse::<usize>().unwrap())
                .collect();
            Button::new(sequence)
        })
        .collect();

    let req_str = parts[2].trim();

    let joltage_req: Vec<Machine> = req_str
        .split(',')
        .map(|n| Machine {
            level: n.trim().parse::<usize>().unwrap()
        } )
        .collect();

    MachineSequence::new(joltage_req, buttons)
}

fn main() {
    let content: String = read_txt_file("input.txt");

    let puzzles: Vec<MachineSequence> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_line(line))
        .collect();

    let total_count: usize = puzzles.iter().enumerate().map(|(i, p)| {
        let res = p.step_to_get_to_goal();
        println!("{} -> {}",i, res);
        res
    }).sum();

    println!("Total count: {}", total_count);
}