mod dfa;
pub mod min_dfa;
mod nfa;
use std::{env};

use min_dfa::min_dfa;
use nfa::Edge;
use petgraph::visit::EdgeRef;
use petgraph::{dot::Dot, prelude::DiGraph};

use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();

    if let Some(raw) = &args.nth(1) {
        if let Some(comb) = is_valid_regex(raw) {
            println!("Invalid regex string - contains: {}", comb);
            return;
        }
        let input = raw.replace(" ", "");
        let nfa = nfa::nfa(input);
        // println!("{:?}", Dot::new(&nfa));
        let dfa = dfa::dfa(&nfa, nfa.node_indices().next().unwrap().index() as u32);
        // println!("{:?}", Dot::new(&dfa));
        let min_dfa = min_dfa(dfa);
        println!("{:?}", Dot::new(&min_dfa));
        to_xml(min_dfa);
    } else {
        println!("No Input Provided...")
    }
}

fn is_valid_regex(regex_str: &str) -> Option<&str> {
    let invalid_combinations = &["**", "++", "?*", "*?", "+*", "*+", "|*", "|+", "|?"];
    for comb in invalid_combinations {
        if regex_str.contains(comb) {
            return Some(comb);
        }
    }
    None
}

fn to_xml(min_dfa: DiGraph<bool, Edge>) {
    let mut file = File::create("out.xml").unwrap();
    let mut tab_count = 1;
    let mut states = String::new();
    tab_count += 1;
    for n in min_dfa.node_indices() {
        let mut tabs = String::new();
        for _ in 0..tab_count {
            tabs.push('\t');
        }
        let state = format!(
            "{}<{}>{}</{}>\n",
            tabs,
            n.index() as u32,
            min_dfa.node_weight(n.clone()).unwrap(),
            n.index() as u32
        );
        states.push_str(&state);
    }
    states = states[..states.len() - 1].to_string();

    let mut transitions = String::new();
    for n in min_dfa.node_indices() {
        let mut trans = String::new();
        let mut edges = min_dfa.edges(n);
        while let Some(edge) = edges.next() {
            let target = edge.target();
            let weight = match edge.weight().clone() {
                Edge::Epsilon => 'ε',
                Edge::Literal(c) => c,
            };
            tab_count += 1;
            let mut tabs = String::new();
            for _ in 0..tab_count {
                tabs.push('\t');
            }
            let part = format!("{0}<{1}>{2}</{1}>\n", tabs, target.index(), weight);
            trans.push_str(&part);
            tab_count -= 1;
        }
        let mut tabs_one = String::new();
        for _ in 0..tab_count {
            tabs_one.push('\t');
        }

        let mut tabs_two = tabs_one.clone();
        tabs_two.push('\t');
        let part = format!("{0}<{1}>\n{2}{0}</{1}>\n", tabs_one, n.index(), trans);
        transitions.push_str(&part);
    }
    transitions = transitions[..transitions.len() - 1].to_string();
    let xml = format!(
        r#"<mindfa>
    <states>
{}
    </states>
    <transitions>
{}
    </transitions>
</mindfa>"#,
        states, transitions
    );
    file.write_all(xml.as_bytes()).unwrap();
}
