use crate::{table::Table, token::Token};
use petgraph::{graph::NodeIndex, prelude::DiGraph};

const TERMINALS: [&'static str; 47] = [
    ",", "p", "{", "}", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ";", "h", "c", ":=", "w",
    "(", ")", "i", "t", "e", "n", "b", "s", "a", "m", "d", "0.00", "-", ".", "T", "F", "^", "v",
    "!", "E", "<", ">", "\"", "*", "g", "o", "r", "$",
];

const NON_TERMINAL: [&'static str; 32] = [
    "PROGR", "PROCDEFS", "PROC", "DIGITS", "D", "MORE", "ALGO", "SEQ", "INSTR", "CALL", "ASSIGN",
    "LOOP", "BRANCH", "ELSE", "NUMVAR", "BOOLVAR", "STRINGV", "NUMEXPR", "DECNUM", "NEG", "POS",
    "INT", "BOOLEXPR", "LOGIC", "CMPR", "STRI", "COMMENT", "C", "INPUT", "OUTPUT", "VALUE", "TEXT",
];

pub struct Parser {
    tokens: Vec<Token>,
    table: Table,
    stack: Vec<NodeIndex>,
    ast: DiGraph<TreeNode, ()>,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub val: String,
    pub is_leaf: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            table: Table::new(),
            stack: Vec::new(),
            ast: DiGraph::new(),
        }
    }

    pub fn parse(&mut self) -> DiGraph<TreeNode, ()> {
        self.init_parser();

        while !self.stack.is_empty() {
            let stack_top = &self.stack_top().val;
            let term = self.tokens[0].lexeme.chars().next();
            if let Some(term) = term {
                let rule = if term == ':' {
                    self.table
                        .get_from_table(&stack_top, &self.tokens[0].lexeme)
                        .to_owned()
                } else if term == '0' && (stack_top == &"NUMEXPR" || stack_top == &"DECNUM") {
                    self.table.get_from_table(&stack_top, "0.00").to_owned()
                } else {
                    self.table.get_from_table(&stack_top, &term.to_string())
                };

                // self._print_stack();
                // println!("LEX ->{:?}", &self.tokens[0].lexeme);
                
                // println!("--------------------------");
                if TERMINALS.contains(&stack_top.as_str()) || stack_top == &"C" {
                    if self.match_stacktop() {
                        self.stack.pop();
                    } else {
                        panic!("\x1B[31m{}\x1B[0m", "PARSE ERROR - FAILED TO MATCH STACKTOP")
                    }
                } else if let Some(rhs) = rule {
                    let term = self.stack.pop().unwrap();
                    let children = self.make_nodes(rhs);

                    self.add_children(term, children.clone());
                    self.push_list(children);
                } else if let None = rule {
                    panic!("\x1B[31m{}\x1B[0m", "PARSE ERROR - NO TABLE RULE FOR INPUT")
                }
            } else {
                panic!("\x1B[31m{}\x1B[0m", "PARSE ERROR - INPUT EMPTY BEFORE STACK")
            }
        }

        self.ast.clone()
    }

    fn init_parser(&mut self) {
        let first = "PROGR";
        let tokens = self.make_nodes(first);
        for token in tokens {
            self.stack.push(token)
        }
    }

    fn stack_top(&self) -> TreeNode {
        let index = self.stack.last().unwrap().to_owned();
        self.ast.node_weight(index).unwrap().to_owned()
    }
    fn push_list(&mut self, children: Vec<NodeIndex>) {
        for child in children {
            self.stack.push(child)
        }
    }

    fn match_stacktop(&mut self) -> bool {
        let stack_top = self.stack_top();
        let stack_top_index = self.stack.last().unwrap().to_owned();
        let mut tokens = self.tokens.clone();

        // println!("MATCH -> {} - {}", stack_top.val, tokens[0].lexeme);
        // println!("--------------------------");

        if stack_top.val == "C" {
            let term = self.tokens[0].lexeme.chars().next().unwrap().to_string();

            if term.is_ascii() {
                let leaf = tokens[0].lexeme.remove(0);
                self.tokens = tokens;
                let leaf = self.ast.add_node(TreeNode {
                    val: leaf.to_string(),
                    is_leaf: true,
                });
                self.ast.add_edge(stack_top_index, leaf, ());
                return true;
            }
        }

        if tokens[0].lexeme.eq(&stack_top.val) {
            let leaf = tokens.remove(0);
            self.tokens = tokens;
            let leaf = self.ast.add_node(TreeNode {
                val: leaf.lexeme.to_string(),
                is_leaf: true,
            });
            self.ast.add_edge(stack_top_index, leaf, ());
            return true;
        }

        if tokens[0].lexeme.starts_with(&stack_top.val) {
            let leaf = tokens[0].lexeme.remove(0);
            self.tokens = tokens;
            let leaf = self.ast.add_node(TreeNode {
                val: leaf.to_string(),
                is_leaf: true,
            });
            self.ast.add_edge(stack_top_index, leaf, ());
            return true;
        }

        false
    }

    fn make_nodes(&mut self, production: &'static str) -> Vec<NodeIndex> {
        let mut tokens = production.split_whitespace().collect::<Vec<&str>>();
        tokens.reverse();

        let mut children = Vec::new();

        for token in tokens {
            let node = self.ast.add_node(TreeNode {
                val: String::from(token),
                is_leaf: match NON_TERMINAL.contains(&token) {
                    true => false,
                    false => true,
                },
            });
            children.push(node);
        }

        children
    }

    fn add_children(&mut self, terminal: NodeIndex, children: Vec<NodeIndex>) {
        for child in children {
            self.ast.add_edge(terminal, child, ());
        }
    }

    fn _print_stack(&mut self) {
        let stack = self.stack.clone();

        let mut normal = Vec::new();
        for index in stack {
            normal.push(self.ast.node_weight(index).unwrap());
        }

        println!("STACK -> {:?}", normal);
    }
}
