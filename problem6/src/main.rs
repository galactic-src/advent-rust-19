use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap};


fn main() {
    let input: Vec<(String, String)> = read_input();
    let mut nodes: HashMap<&str, (&str, Vec<&str>)> = HashMap::new();

    for edge in &input {
        nodes.entry(&edge.0)
            .and_modify(|(_parent, children)|children.push(&edge.1))
            .or_insert((&"", vec!(&edge.1)));

        nodes.entry(&edge.1)
            .and_modify(|val|val.0 = &edge.0)
            .or_insert((&edge.0,vec!()));
    }

    part1(&nodes);
    part2(&nodes);
}

fn part1(nodes: &HashMap<&str, (&str, Vec<&str>)>) {
    let descendants = sum_count_descendants(&nodes);
    println!("part 1: {}", descendants);
}

fn part2(nodes: &HashMap<&str, (&str, Vec<&str>)>) {
    let santa_path = path_to("SAN", nodes);
    let you_path = path_to("YOU", nodes);

    let common: usize = santa_path.iter()
        .zip(&you_path).filter(|(san, you)| san == you)
        .count() as usize;
    let total = santa_path.len() - common + you_path.len() - common;
    println!("part 2: {}", total);
}

fn path_to<'a>(name: &'a str, nodes: &'a HashMap<&str, (&str, Vec<&str>)>) -> Vec<&'a str> {
    let mut path = vec!();
    let mut current = name;
    while current != "COM" {
        current = nodes[current].0;
        path.push(current);
    }
    path.reverse();
    path
}

fn sum_count_descendants(map: &HashMap<&str, (&str, Vec<&str>)>) -> u64 {
    map.keys().map(|name| tally_descendants_recursive(map, name))
        .sum()
}

fn tally_descendants_recursive(map: &HashMap<&str, (&str, Vec<&str>)>, name: &str) -> u64 {
    let descendants: u64 = map[name].1.iter()
        .map(|descendant| tally_descendants_recursive(map, descendant))
        .sum();
    descendants + map[name].1.len() as u64
}

fn read_input<'a>() -> Vec<(String, String)> {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line.expect("failed to read line");
            let pair: Vec<&str> = line.split(')').collect();
            (String::from(pair[0]), String::from(pair[1]))
        })
        .collect()
}