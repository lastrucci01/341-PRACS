use petgraph::graph::{DiGraph, NodeIndex};
use regex_syntax::{
    hir::{self, Hir},
    Parser,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Edge {
    Epsilon,
    Literal(char),
}

pub fn nfa(input: String) -> DiGraph<bool, Edge> {
    let nfa = &mut DiGraph::<bool, Edge>::new();
    let regex = Parser::new().parse(&input).unwrap();
    // dbg!(regex.clone());
    sub_nfa(nfa, &regex, true).unwrap();

    nfa.to_owned()
}

fn sub_nfa(
    graph: &mut DiGraph<bool, Edge>,
    regex: &Hir,
    end_true: bool,
) -> Result<(NodeIndex, NodeIndex), &'static str> {
    match regex.to_owned().into_kind() {
        hir::HirKind::Literal(l) => match l {
            hir::Literal::Unicode(c) => {
                let start = graph.add_node(false);
                let end = accept(end_true, graph);
                graph.add_edge(start, end, Edge::Literal(c));
                Ok((start, end))
            }
            hir::Literal::Byte(_) => Err("Invalid literal format"),
        },
        hir::HirKind::Alternation(alts) => {
            let (start, end) = (graph.add_node(false), accept(end_true, graph));
            let mut alt_nfas: Vec<(NodeIndex, NodeIndex)> = Vec::new();
            for alt in alts.iter() {
                alt_nfas.push(sub_nfa(graph, alt, false).unwrap());
            }

            for nfa in alt_nfas {
                graph.add_edge(start, nfa.0, Edge::Epsilon);
                graph.add_edge(nfa.1, end, Edge::Epsilon);
            }
            Ok((start, end))
        }
        hir::HirKind::Repetition(rep) => match rep.kind {
            hir::RepetitionKind::ZeroOrOne => {
                let (start, end) = (graph.add_node(false), accept(end_true, graph));
                let (one_start, one_end) = sub_nfa(graph, rep.hir.as_ref(), false).unwrap();

                graph.add_edge(start, end, Edge::Epsilon);
                graph.add_edge(start, one_start, Edge::Epsilon);
                graph.add_edge(one_end, end, Edge::Epsilon);
                Ok((start, end))
            }
            hir::RepetitionKind::ZeroOrMore => {
                let index = accept(end_true, graph);
                let (inner_start, inner_end) = sub_nfa(graph, rep.hir.as_ref(), false).unwrap();
                graph.add_edge(index, inner_start, Edge::Epsilon);
                graph.add_edge(inner_end, index, Edge::Epsilon);

                Ok((index, index))
            }
            hir::RepetitionKind::OneOrMore => {
                let (start, end) = (graph.add_node(false), accept(end_true, graph));
                let (one_start, one_end) = sub_nfa(graph, rep.hir.as_ref(), false).unwrap();

                graph.add_edge(start, one_start, Edge::Epsilon);
                let more = graph.add_node(false);

                graph.add_edge(one_end, more, Edge::Epsilon);
                let (more_start, more_end) = sub_nfa(graph, rep.hir.as_ref(), false).unwrap();

                graph.add_edge(more, more_start, Edge::Epsilon);
                graph.add_edge(more_end, more, Edge::Epsilon);
                graph.add_edge(more, end, Edge::Epsilon);

                Ok((start, end))
            }
            hir::RepetitionKind::Range(_) => Err("Range repetition is not allowed"),
        },
        hir::HirKind::Group(group) => match group.kind {
            hir::GroupKind::CaptureIndex(_) => sub_nfa(graph, group.hir.as_ref(), end_true),
            hir::GroupKind::CaptureName { name: _, index: _ } => {
                Err("CaptureName groups not allowed")
            }
            hir::GroupKind::NonCapturing => Err("NonCapturing groups not allowed"),
        },
        hir::HirKind::Concat(concats) => {
            let (mut start, end) = (graph.add_node(false), accept(end_true, graph));
            let ret_start = start;
            for (i, concat) in concats.iter().enumerate() {
                let (concat_start, concat_end) = sub_nfa(graph, concat, false).unwrap();
                graph.add_edge(start, concat_start, Edge::Epsilon);
                start = concat_end;
                if i == concats.len() - 1 {
                    graph.add_edge(start, end, Edge::Epsilon);
                }
            }
            Ok((ret_start, end))
        }
        hir::HirKind::Empty => Err("Epislon not allowed"),
        hir::HirKind::Class(_) => Err("Classes not allowed"),
        hir::HirKind::Anchor(_) => Err("Anchor tags not allowed."),
        hir::HirKind::WordBoundary(_) => Err("Word boundaries not allowed."),
    }
}

pub fn accept(end_true: bool, graph: &mut DiGraph<bool, Edge>) -> NodeIndex {
    if end_true {
        graph.add_node(true)
    } else {
        graph.add_node(false)
    }
}
