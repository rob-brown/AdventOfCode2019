use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn calculate_orbit_depth(node: &String, depth: i32, orbits: &HashMap<String, Vec<String>>) -> i32 {
    let mut total_depth = depth;

    if let Some(satelites) = orbits.get(node) {
        for s in satelites {
            total_depth += calculate_orbit_depth(s, depth + 1, &orbits);
        }
    }

    total_depth
}

fn path_to(
    node: &String,
    current: &String,
    path: &mut Vec<String>,
    child_to_parent: &HashMap<String, String>,
) {
    if current != "COM" {
        let inner = child_to_parent.get(current).unwrap();
        path.push(inner.to_string());
        path_to(node, inner, path, &child_to_parent)
    }
}

fn transit_length(path1: Vec<String>, path2: Vec<String>) -> usize {
    let mut set1: HashSet<String> = HashSet::with_capacity(path1.len());
    let mut set2: HashSet<String> = HashSet::with_capacity(path2.len());

    for x in path1 { set1.insert(x); }
    for x in path2 { set2.insert(x); }

    let intersection = set1.intersection(&set2).count();

    set1.len() - intersection + set2.len() - intersection
}

pub fn solve() {
    let mut file = File::open("input/day6.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let regex = Regex::new(r"([\w\d]{3})\)([\w\d]{3})").unwrap();
    let mut orbits: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_to_parent: HashMap<String, String> = HashMap::new();

    for capture in regex.captures_iter(&contents) {
        let inner = capture[1].parse::<String>().unwrap();
        let outer = capture[2].parse::<String>().unwrap();
        let list = orbits.entry(inner.clone()).or_insert(Vec::new());
        list.push(outer.clone());
        child_to_parent.insert(outer, inner);
    }

    let root = String::from("COM");
    let you = String::from("YOU");
    let santa = String::from("SAN");

    println!("Day 6:A = {}", calculate_orbit_depth(&root, 0, &orbits));

    let mut you_path = Vec::new();
    path_to(&you, &you, &mut you_path, &child_to_parent);

    let mut santa_path = Vec::new();
    path_to(&santa, &santa, &mut santa_path, &child_to_parent);

    println!("Day 6:B = {}", transit_length(you_path, santa_path));
}
