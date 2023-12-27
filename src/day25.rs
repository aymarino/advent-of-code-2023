use std::collections::{hash_map::Entry, HashMap, HashSet};

pub fn soln() -> (u64, u64) {
    let input = include_str!("../input/25.txt");
    let mut edges = HashMap::<&str, HashSet<_>>::new();
    input.lines().for_each(|line| {
        let (first, list) = line.split_once(": ").unwrap();
        let rest = list.split(' ').collect::<Vec<_>>();
        match edges.entry(first) {
            Entry::Occupied(mut v) => {
                v.get_mut().extend(rest.clone());
            }
            Entry::Vacant(e) => {
                e.insert(HashSet::from_iter(rest.iter().cloned()));
            }
        }
        for r in &rest {
            match edges.entry(r) {
                Entry::Occupied(mut v) => {
                    v.get_mut().insert(first);
                }
                Entry::Vacant(e) => {
                    e.insert(HashSet::from([first]));
                }
            }
        }
    });

    // Find a graph component C s.t. there are 3 connections to the rest of the graph (G / C).
    // Iteratively remove nodes from the component that have many connections (i.e. the most
    // connections).
    let mut component = HashSet::from_iter(edges.keys().cloned());
    while component
        .iter()
        .map(|&k| edges.get(k).unwrap().difference(&component).count())
        .sum::<usize>()
        != 3
    {
        let max_outbound_connections = component
            .iter()
            .max_by_key(|&k| edges.get(k).unwrap().difference(&component).count())
            .unwrap();
        component.remove(*max_outbound_connections);
    }
    let p1 = component.len()
        * HashSet::from_iter(edges.keys().cloned())
            .difference(&component)
            .count();
    (p1 as u64, 0)
}
