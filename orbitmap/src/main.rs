extern crate orbitmap;

use std::fs::File;
use std::io::{BufReader, BufRead};

use orbitmap::Tree;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut orbits: Vec<(String, String)> = Vec::new();
    for line in reader.lines() {
        let mut bodies: Vec<String> = line.expect("error reading line").trim()
            .split(')')
            .map(|x| x.into())
            .collect();
        let orbitee = bodies.pop().expect("input not pairs");
        let orbiter = bodies.pop().expect("input not pairs");
        orbits.push((orbiter, orbitee));
    }
    let t = Tree::new(orbits);
    println!("DFS depth total: {}", t.dfs_depth_total());
}
