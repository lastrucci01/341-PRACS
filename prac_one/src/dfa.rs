use std::collections::{HashMap, HashSet};

use crate::nfa::Edge;
use petgraph::{adj::NodeIndex, prelude::DiGraph, visit::EdgeRef};
pub fn dfa(graph: &DiGraph<bool, Edge>, start: NodeIndex) -> DiGraph<bool, Edge> {
    let accepts = find_accepts(graph); // Get the accept states of the nfa
    let literals = find_literals(graph); // get the alphabet over the nfa

    let mut dfa = DiGraph::<bool, Edge>::new(); // init the new dfa
    let mut states = HashMap::<Vec<u32>, (_, bool)>::new(); // init the states Hashmap
                                                            // the key is a unique vector of nfa states that represent a dfa state
                                                            // the value is a tuple storing:
                                                            //  - NodeIndex of a dfa state
                                                            //  - bool value representing if the accept states are in the vector of nfa states
    let mut visited = Vec::<Vec<u32>>::new(); //what states have been visited

    let closure = &mut Vec::<u32>::new(); // for consumption by eps close function
    let start = epsilon_closure(closure, graph, start); // start state of dfa
    let start_accept = check_accept(&accepts, start.to_vec()); // is start accept?
    let start_index = dfa.add_node(start_accept); // new node for start
    states.insert(start.to_vec(), (start_index, start_accept)); // add to Hashmap of states

    let mut keys = states
        .keys()
        .map(|k| k.to_owned())
        .collect::<Vec<Vec<u32>>>();
    //keys of states hashmap, i.e., list of vectors of nfa states representing the dfa states
    while !keys.eq(&visited) {
        // while keys and visisted do not equal
        let not_visited = states
            .keys()
            .filter_map(|k| {
                if !visited.contains(k) {
                    Some(k.to_owned())
                } else {
                    None
                }
            })
            .collect::<Vec<Vec<u32>>>();

        if !not_visited.is_empty() {
            visited.push(not_visited[0].clone());
            for literal in &literals {
                let res = move_literal(&literal, not_visited[0].clone(), graph);
                let state = states.get(&not_visited[0]).unwrap().0;
                if let Some(new_state) = res {
                    let entry = states.entry(new_state.clone()).or_insert_with(|| {
                        let accept = check_accept(&accepts, new_state.to_vec());
                        let index = dfa.add_node(accept);
                        (index, accept)
                    });
                    dfa.add_edge(state.to_owned(), entry.0, Edge::Literal(*literal));
                }
            }
        }

        keys = states
            .keys()
            .map(|k| k.to_owned())
            .collect::<Vec<Vec<u32>>>(); // update keys
        visited.sort(); // sort visisted
        keys.sort(); // sort keys
    }

    dfa
}

fn move_literal(literal: &char, state: Vec<u32>, graph: &DiGraph<bool, Edge>) -> Option<Vec<u32>> {
    let source_states = source_states(state, literal, graph);
    let mut state = &mut Vec::<NodeIndex>::new();

    for s in source_states {
        state = epsilon_closure(state, graph, s);
    }

    if !state.is_empty() {
        Some(state.to_owned())
    } else {
        None
    }
}

fn epsilon_closure<'a>(
    closure: &'a mut Vec<NodeIndex>,
    graph: &'a DiGraph<bool, Edge>,
    start: NodeIndex,
) -> &'a mut Vec<NodeIndex> {
    let mut edges = graph.edges(start.into());
    if !closure.contains(&start) {
        closure.push(start)
    }
    while let Some(edge) = edges.next() {
        if edge.weight() == &Edge::Epsilon && !closure.contains(&(edge.target().index() as u32)) {
            closure.push(edge.target().index() as u32);
            epsilon_closure(closure, graph, edge.target().index() as u32);
        }
    }
    closure
}

fn source_states(state: Vec<u32>, literal: &char, graph: &DiGraph<bool, Edge>) -> Vec<u32> {
    graph
        .edge_references()
        .filter_map(|e| {
            if e.weight() == &Edge::Literal(literal.clone())
                && state.contains(&(e.source().index() as u32))
            {
                Some(e.target().index() as u32)
            } else {
                None
            }
        })
        .collect::<Vec<u32>>()
}

fn find_accepts(graph: &DiGraph<bool, Edge>) -> Vec<u32> {
    graph
        .node_indices()
        .filter_map(|n| match graph.node_weight(n).unwrap() {
            true => Some(n.index() as u32),
            false => None,
        })
        .collect::<Vec<u32>>()
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

fn check_accept(accept: &Vec<u32>, state: Vec<u32>) -> bool {
    for s in &state {
        if accept.contains(&s) {
            return true;
        }
    }
    false
}
