extern crate regex;

use std::collections::HashMap;

pub fn construct_tower(input: Vec<String>) -> (HashMap<String, String>, HashMap<String, u32>, HashMap<String, Vec<String>>) {
    let mut parents = HashMap::new();
    let mut weights = HashMap::new();
    let mut children = HashMap::new();
    let definitions:Vec<Vec<String>> = input.iter().map(|s| s.split_whitespace().map(|x| String::from(x)).collect()).collect();
    for def in definitions {
        if def.len() > 2 {
            let mut i = 3;
            let mut child_names = Vec::new();
            while i < def.len() {
                let name = def[i].clone().split(",").map(|x| String::from(x)).collect::<Vec<String>>()[0].clone();
                parents.insert(name.clone(), def[0].clone());
                child_names.push(name.clone());
                i += 1;
            }
            children.insert(def[0].clone(), child_names);
            weights.insert(def[0].clone(), parse_weight(def[1].clone()));
        } else {
            weights.insert(def[0].clone(), parse_weight(def[1].clone()));
        }
    }
    (parents, weights, children)
}

fn parse_weight(text: String) -> u32 {
    let re = regex::Regex::new(r"\D").unwrap();
    let weight = re.replace_all(&text, "").to_string();
    u32::from_str_radix(&weight, 10).unwrap()
}

pub fn find_weight_at_node(tower: HashMap<String, Vec<String>>, weights: HashMap<String, u32>, node: String) -> u32 {
    let own_weight = weights.get(&node).unwrap();
    let children:Vec<String> = if tower.contains_key(&node) {
        (*tower.get(&node).unwrap()).clone()
    } else {
        Vec::new()
    };
    if children.len() > 0 {
        *own_weight + children.iter().fold(0, |acc, x| acc + find_weight_at_node(tower.clone(), weights.clone(), (x.to_string()).clone()))
    } else {
        *own_weight
    }
}

pub fn print_node_weights(tower: HashMap<String, Vec<String>>, weights: HashMap<String, u32>, root: String) {
    println!("Parent: {}", root);
    if !tower.contains_key(&root) {
        return
    }
    let children:Vec<String> = (*tower.get(&root).unwrap()).clone();
    for child in children.clone() {
        println!("Child: {} ({}): {}", child, weights.get(&child).unwrap(), find_weight_at_node(tower.clone(), weights.clone(), child.clone()));
    }
    for child in children {
        print_node_weights(tower.clone(), weights.clone(), child.clone());
    }
}

pub fn find_root_of_tower(tower: HashMap<String, String>) -> String {
    let (key, _) = tower.iter().next().unwrap();
    let mut current = key.clone();
    loop {
        if !tower.contains_key(&current) {
            return current;
        } else {
            current = tower.get(&current).unwrap().clone();
        }
    }
}
