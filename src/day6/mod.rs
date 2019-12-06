use crate::util::read_input;
use std::collections::{HashMap, HashSet};
use std::io::{prelude::*, BufReader};

pub fn day_6() {
    let file = read_input(6);
    let reader = BufReader::new(file);

    let mut orbits: HashMap<String, String> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let split: Vec<&str> = line.split(')').collect();
        let orbitee = split[0];
        let orbiter = split[1];

        orbits.insert(orbiter.to_owned(), orbitee.to_owned());
    }

    // Part 1
    let mut checksum = 0;
    for node in orbits.keys() {
        checksum += get_hops_to_ancestor(&orbits, node, "COM");
    }

    println!("Day 6-1: {}", checksum);

    // Part 2
    let ancestor = find_first_common_ancestor(&orbits, "YOU", "SAN");
    println!(
        "Day 6-2: {}",
        get_hops_to_ancestor(&orbits, orbits.get("YOU").unwrap(), &ancestor)
            + get_hops_to_ancestor(&orbits, orbits.get("SAN").unwrap(), &ancestor)
    );
}

fn get_ancestors<'a>(orbits: &'a HashMap<String, String>, mut node: &'a str) -> HashSet<&'a str> {
    let mut ancestors: HashSet<&str> = HashSet::new();

    loop {
        if let Some(parent) = orbits.get(node) {
            ancestors.insert(parent);
            node = parent;
        } else {
            break;
        }
    }

    ancestors
}

fn find_first_common_ancestor(orbits: &HashMap<String, String>, a: &str, b: &str) -> String {
    let ancestors_a = get_ancestors(orbits, a);
    let ancestors_b = get_ancestors(orbits, b);

    let intersection: Vec<&str> = ancestors_a
        .intersection(&ancestors_b)
        .map(|ancestor| *ancestor)
        .collect();

    let mut fewest_hops = usize::max_value();
    let mut ancestor: &str = "";
    for node in intersection.iter() {
        let hops = get_hops_to_ancestor(orbits, a, node);
        if hops < fewest_hops {
            fewest_hops = hops;
            ancestor = node;
        }
    }

    ancestor.to_owned()
}

fn get_hops_to_ancestor(orbits: &HashMap<String, String>, start: &str, ancestor: &str) -> usize {
    let mut hops = 0;

    let mut node = start;
    loop {
        if let Some(parent) = orbits.get(node) {
            hops += 1;
            node = parent;

            if node == ancestor {
                break;
            }
        } else {
            break;
        }
    }

    hops
}
