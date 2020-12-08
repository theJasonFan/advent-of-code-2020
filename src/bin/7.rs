use aoc::*;
use std::env;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(args.last().unwrap()).unwrap();

    let mut dag = HashMap::new();
    for line in lines {
        if let Ok(line) = line {
            let bag = parse_line(&line);
            dag.insert(bag.color.clone(), bag);
        }
    }

    let mut ans = Vec::new();

    let source = "shiny gold";

    for k in dag.keys() {
        if contains(&dag, &k, source) {
            ans.push(k.clone());
        }
    }
    println!("{:?}", ans.len());
    println!("{:?}", contains_n(&dag, source));
}

pub type BagChild = (usize, String);
#[derive(Debug)]
pub struct Bag {
    color: String,
    children: Option<Vec<BagChild>>,
}

pub fn parse_line(line: &str) -> Bag {
    if line.ends_with("contain no other bags.") {
        let tokens: Vec<&str> = line.split(" ").collect();
        let color = tokens[0..2].join(" ");
        Bag {
            color,
            children: None,
        }
    } else {
        //println!("hello");
        let tokens: Vec<&str> = line.split(" ").collect();
        let color = tokens[0..2].join(" ");
        let mut children = Vec::<BagChild>::new();

        let counts =
            tokens[4..]
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if (i % 4) == 0 { Some(*e) } else { None });
        let colors_pref =
            tokens[4..]
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if (i % 4) == 1 { Some(*e) } else { None });
        let colors_suff =
            tokens[4..]
                .iter()
                .enumerate()
                .filter_map(|(i, e)| if (i % 4) == 2 { Some(*e) } else { None });
        let colors = colors_pref
            .zip(colors_suff)
            .map(|(p, s)| format!("{} {}", p, s));

        for (count, color) in counts.zip(colors) {
            let count: usize = count.parse().unwrap();
            children.push((count, color.to_string()))
        }
        let children = Some(children);

        Bag { color, children }
    }
}

pub fn contains(dag: &HashMap<String, Bag>, source: &str, pattern: &str) -> bool {
    let this = dag.get(source).unwrap();
    match &this.children {
        None => false,
        Some(children) => {
            let any = children
                .iter()
                .any(|(_, color)| color == pattern || contains(dag, color, pattern));
            any
        }
    }
}

pub fn contains_n(dag: &HashMap<String, Bag>, source: &str) -> usize {
    let this = dag.get(source).unwrap();
    match &this.children {
        None => 0,
        Some(children) => {
            let mut tot = 0;
            for (count, color) in children {
                tot += count;
                tot += count * contains_n(dag, color)
            }
            tot
        }
    }
}
