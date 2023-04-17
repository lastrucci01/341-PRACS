use std::{fs::File, io::Write};

use petgraph::{
    adj::NodeIndex, prelude::DiGraph, stable_graph::IndexType, visit::EdgeRef, Direction::Incoming,
};

use crate::parser::TreeNode;

#[derive(Debug)]
pub struct AST {
    ast: DiGraph<TreeNode, ()>,
}

impl AST {
    pub fn new(ast: DiGraph<TreeNode, ()>) -> Self {
        Self { ast }
    }

    pub fn prune(&mut self) {
        let mut indices = self.ast.node_indices();
        while let Some(node) = indices.next() {
            if let Some(treenode) = self.ast.node_weight(node) {
                let leaf = treenode.is_leaf;
                let child_count = self.ast.edges(node).collect::<Vec<_>>().len();

                if child_count == 0 && !leaf {
                    self.ast.remove_node(node);
                }
            }
        }
    }

    pub fn to_xml(&self) {
        let tab_index: usize = 0;
        let mut file = File::create("ast.xml").unwrap();
        if let Some(root) = self.root() {
            let node = self.ast.node_weight(root.into()).unwrap();
            let start = format!("<{} id=\"{}\">\n", node.val, root);
            file.write(start.as_bytes()).unwrap();
            self.expand_children(root, &mut file, tab_index + 1);
            let end = format!("</{}>\n", node.val);
            file.write(end.as_bytes()).unwrap();
        }
    }

    fn root(&self) -> Option<NodeIndex> {
        for node in self.ast.node_indices() {
            if self.ast.edges_directed(node, Incoming).next().is_none() {
                return Some(node.index() as NodeIndex);
            }
        }
        None
    }

    fn expand_children(&self, index: NodeIndex, file: &mut File, tab_index: usize) {
        let tabs = "\t".repeat(tab_index);

        let children: Vec<NodeIndex> = self
            .ast
            .edges(index.into())
            .map(|e| e.target().index() as NodeIndex)
            .collect();

        if children.len() == 1 && self.ast.node_weight(children[0].into()).unwrap().is_leaf {
            let leaf = self.ast.node_weight(children[0].into()).unwrap();
            let start = format!("{}{}\n", tabs, leaf.val);
            file.write(start.as_bytes()).unwrap();
        } else {
            for child in children {
                let node = self.ast.node_weight(child.into()).unwrap();
                if node.is_leaf {
                    self.expand_children(child.index() as u32, file, tab_index + 1);
                } else {
                    let start = format!("{}<{} id=\"{}\">\n", tabs, node.val, child);
                    file.write(start.as_bytes()).unwrap();
                    self.expand_children(child.index() as u32, file, tab_index + 1);
                    let end = format!("{}</{}>\n", tabs, node.val);
                    file.write(end.as_bytes()).unwrap();
                }
            }
        }
    }
}
