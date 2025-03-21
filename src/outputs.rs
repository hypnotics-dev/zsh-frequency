use std::{collections::HashMap, usize};

pub fn top(map: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| b.1.cmp(a.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, val)| (key.clone(), *val))
        .collect::<Vec<(String, usize)>>()
}

pub fn bot(map: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut origin: Vec<_> = map.iter().collect();
    origin.sort_by(|a, b| a.1.cmp(b.1));

    origin
        .into_iter()
        .take(n)
        .map(|(key, val)| (key.clone(), *val))
        .collect::<Vec<(String, usize)>>()
}

pub fn rand(map: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    map.keys()
        .into_iter()
        .take(n)
        .map(|k| (String::from(k), map[k]))
        .collect()
}

pub fn get(map: HashMap<String, usize>, key: String) -> (String, usize) {
    let val = match map.get(key.as_str()) {
        Some (x) => {*x}
        None => {0}
    };
    (key.to_string(), val)
}

pub fn print(vector: Vec<(String, usize)>) {
    vector.into_iter().for_each(|p| println!("{}:{}", p.0, p.1))
}
