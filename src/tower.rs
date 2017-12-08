use std::collections::HashMap;

pub fn construct_tower(input: Vec<String>) -> (HashMap<String, String>, HashMap<String, String>) {
    let mut tower = HashMap::new();
    let mut weights = HashMap::new();
    let definitions:Vec<Vec<String>> = input.iter().map(|s| s.split_whitespace().map(|x| String::from(x)).collect()).collect();
    for def in definitions {
        if def.len() > 2 {
            let mut i = 3;
            while i < def.len() {
                let name = def[i].clone().split(",").map(|x| String::from(x)).collect::<Vec<String>>()[0].clone();
                tower.insert(name, def[0].clone());
                i += 1;
            }
        } else {
            weights.insert(def[0].clone(), def[1].clone());
        }
    }
    (tower, weights)
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
