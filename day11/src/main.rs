use hashbrown::{HashMap, HashSet};
use std::fs;

fn read_txt_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| panic!("Could not open file: {}", path))
}



fn visit_next<'a>(past: Vec<&'a str>, past_set: HashSet<&str>, map: &hashbrown::HashMap<&str, Vec<&'a str>>, memory: &mut HashMap<&'a str, usize>, to: &str) -> usize{
    let current = past.last().unwrap().clone();
    if let Some(r) = memory.get(current) {
        return r.clone()
    }

    if current == to {
        return 1
    }

    if None == map.get(current) {
        return 0
    }

    let s: usize = map.get(current).unwrap().iter().map( |n| {
        let mut visit = past.clone();
        let mut new_set = past_set.clone();
        visit.push(n.clone());
        if new_set.contains(n) {
            return 0
        }
        new_set.insert(n.clone());
        let res = visit_next(visit, new_set, map, memory, to);
        res
    }).sum();

    memory.insert(current, s.clone());

    s
}


fn visit(from: &str, to: &str, nodes: &HashMap<&str, Vec<&str>>, avoid: &Vec<&str>) -> usize{
    let mut memory: HashMap<&str, usize> = HashMap::new();

    for a in avoid {
        memory.insert(a.clone(), 0);
    }

    let result = visit_next([from].to_vec(), [from].to_vec().iter().map(|s| s.clone()).collect(), &nodes, &mut memory, to);

    result

}

fn main() {
    let input: String = read_txt_file("input.txt");


    let nodes: HashMap<&str, Vec<&str>> = input.lines().map(|l| {
        let (code,p1) = l.split_once(':').unwrap();
        let nexts: Vec<&str>= p1.split(' ').collect();
        (code, nexts)
    }).collect();

    let a = visit("svr", "fft", &nodes, &["out", "dac"].to_vec());
    let b = visit("fft", "dac", &nodes, &["out", "svr"].to_vec());
    let c = visit("dac", "out", &nodes, &["fft", "svr"].to_vec());

    let aa = visit("svr", "dac", &nodes, &["out", "fft"].to_vec());
    let bb = visit("dac", "fft", &nodes, &["out", "svr"].to_vec());
    let cc = visit("fft", "out", &nodes, &["dac", "svr"].to_vec());



    println!("{}", a*b*c + aa*bb*cc);
}