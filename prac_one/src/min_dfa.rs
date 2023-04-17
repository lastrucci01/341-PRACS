use std::collections::{HashMap, HashSet};

use petgraph::{adj::NodeIndex, prelude::DiGraph, stable_graph::IndexType, visit::EdgeRef};

use crate::nfa::Edge;

pub fn min_dfa(dfa: DiGraph<bool, Edge>) -> DiGraph<bool, Edge> {
    let mut groups = Vec::<(Vec<NodeIndex>, bool)>::new();
    let start = start_groups(&dfa);
    if !start.0.is_empty() {
        groups.push((start.0, true))
    };
    if !start.1.is_empty() {
        groups.push((start.1, false));
    }

    let mut consistent = Vec::<(Vec<NodeIndex>, bool)>::new();
    while !groups.eq(&consistent) {
        for (i, g) in groups.clone().into_iter().enumerate() {
            let group_info = build_group(&dfa, g.0.clone(), &groups);
            if let Some(new_groups) = check_consistent_group(group_info.clone()) {
                groups.remove(i);
                for new in new_groups {
                    groups.push((new, g.1))
                }
                consistent.clear();
            } else {
                if !consistent.contains(&g) {
                    consistent.push(g.clone())
                }
            }
        }


        groups.sort();
        consistent.sort();
    }

    build_dfa(&dfa, &groups)
}

fn build_dfa(
    dfa: &DiGraph<bool, Edge>,
    groups: &Vec<(Vec<NodeIndex>, bool)>,
) -> DiGraph<bool, Edge> {
    let mut min_dfa = DiGraph::<bool, Edge>::new();
    let mut mappings = Vec::<(NodeIndex, (Vec<NodeIndex>, bool))>::new();
    for g in groups {
        let node = min_dfa.add_node(g.1);
        mappings.push((node.index() as u32, g.clone()));
    }

    for (node_index, group) in mappings.clone() {
        let group_hash = build_group(dfa, group.0, groups)
            .into_iter()
            .next()
            .unwrap()
            .1;
        for (c, group) in group_hash {
            let mut map_iter = mappings.clone().into_iter();
            let other = loop {
                if let Some((other_index, other_group)) = map_iter.next() {
                    if group.eq(&other_group.0) {
                        break Some(other_index);
                    }
                } else {
                    break None;
                }
            };
            if let Some(other_index) = other {
                min_dfa.add_edge((node_index).into(), other_index.into(), Edge::Literal(c));
            }
        }
    }

    min_dfa
}

fn check_consistent_group(
    group_info: HashMap<u32, HashMap<char, Vec<u32>>>,
) -> Option<Vec<Vec<u32>>> {
    let mut groups = Vec::<(Vec<u32>, &HashMap<char, Vec<u32>>)>::new();
    for (index, trans) in &group_info {
        let mut found = false;
        for group in &mut groups {
            if group.1.eq(trans) {
                found = true;
                group.0.push(index.index() as u32);
            }
        }
        if !found {
            groups.push((vec![*index], trans))
        }
    }
    if groups.len() == 1 {
        None
    } else {
        let mut new_groups = Vec::<Vec<u32>>::new();
        for g in groups {
            new_groups.push(g.0);
        }
        Some(new_groups)
    }
}

fn build_group(
    dfa: &DiGraph<bool, Edge>,
    group: Vec<NodeIndex>,
    groups: &Vec<(Vec<NodeIndex>, bool)>,
) -> HashMap<u32, HashMap<char, Vec<u32>>> {
    let alphabet = find_literals(dfa);
    let mut group_info = HashMap::<NodeIndex, HashMap<char, Vec<u32>>>::new();
    for index in group {
        for a in &alphabet {

            let mut info = if group_info.contains_key(&index) {
                group_info.get(&index).unwrap().clone()
            } else {
                let info = HashMap::<char, Vec<u32>>::new();
                group_info.insert(index, info);
                group_info.get(&index).unwrap().clone()
            };
            let edges = dfa
                .edges((index).into())
                .filter_map(|e| {
                    if e.weight() == &Edge::Literal(*a) {
                        Some(e.target())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if edges.is_empty() {
                info.insert(*a, Vec::new());
                group_info.insert(index, info);
            } else {
                let mut groups_iter = groups.into_iter();
                let group = loop {
                    if let Some(g) = groups_iter.next() {
                        let mut i = 0;

                        while i < edges.len() && !g.0.contains(&(edges[i].index() as u32)) {
                            i += 1;
                        }
                        if i == edges.len() {
                            continue;
                        } else {
                            break Some(g);
                        }
                    }
                };
                if let Some(g) = group {
                    info.insert(*a, g.0.to_vec());
                } else {
                    info.insert(*a, Vec::new());
                }

                group_info.insert(index, info);
            }
        }
    }

    group_info
}

fn find_literals(graph: &DiGraph<bool, Edge>) -> HashSet<char> {
    graph
        .edge_references()
        .filter_map(|e| match e.weight() {
            Edge::Epsilon => None,
            Edge::Literal(c) => Some(c.to_owned()),
        })
        .collect::<HashSet<char>>()
}

fn start_groups(dfa: &DiGraph<bool, Edge>) -> (Vec<NodeIndex>, Vec<NodeIndex>) {
    let mut accepts = Vec::<NodeIndex>::new();
    let mut non_accepts = Vec::<NodeIndex>::new();

    let mut ind = dfa.node_indices();
    while let Some(index) = ind.next() {
        if *(dfa.node_weight(index).unwrap()) {
            accepts.push(index.index() as u32)
        } else {
            non_accepts.push(index.index() as u32)
        }
    }

    (accepts, non_accepts)
}
