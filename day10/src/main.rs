use std::collections::VecDeque;
use hashbrown::HashSet;
use std::fs;

fn parse_list(s: &str) -> Vec<usize> {
    s[1..s.len()-1].split(',').map(|w| w.parse().unwrap()).collect()
}

fn solve_p1(buttons: &[Vec<usize>], lights: &str) -> usize {
    let goal = lights.bytes().map(|c| c == b'#').collect::<Vec<_>>();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((vec![false; lights.len()], 0usize));
    while let Some((state, n)) = q.pop_front() {
        if seen.contains(&state) {
            continue;
        }
        if state == goal {
            return n;
        }
        seen.insert(state.clone());
        for button in buttons {
            let mut next = state.clone();
            for &i in button {
                next[i] = !next[i];
            }
            q.push_back((next, n + 1));
        }
    }
    unreachable!()
}

fn solve_p2(buttons: &[Vec<usize>], jolts: &[usize]) -> usize {
    use good_lp::*;
    let mut vars = variables!();
    let press_vars = (0..buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect::<Vec<_>>();

    let mut problem = highs(vars.minimise(press_vars.iter().sum::<Expression>()));
    let mut exprs = vec![0.into_expression(); jolts.len()];
    for i in 0..buttons.len() {
        for &x in &buttons[i] {
            exprs[x] += press_vars[i];
        }
    }
    for (e, &j) in exprs.into_iter().zip(jolts) {
        problem.add_constraint(e.eq(j as f64));
    }
    let sol = problem.solve().unwrap();
    press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as _
}

fn read_txt_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open file: {}", path))
}
fn main() {
    let input: String = read_txt_file("input.txt");

    let machines = input.split('\n').map(|l| {
        let (lights_str, rest) = l.split_once(' ').unwrap();
        let (buttons_str, jolts_str) = rest.rsplit_once(' ').unwrap();
        let lights = lights_str[1..lights_str.len()-1].to_string();
        let buttons = buttons_str.split(' ').map(parse_list).collect::<Vec<_>>();
        (lights, buttons, parse_list(jolts_str))
    }).collect::<Vec<_>>();
    let p1: usize = machines.iter().map(|m| solve_p1(&m.1, &m.0)).sum();
    let p2: usize = machines.iter().map(|m| solve_p2(&m.1, &m.2)).sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

}