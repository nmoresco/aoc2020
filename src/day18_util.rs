use crate::day18_util::Token::{OpenParen, Op, CloseParen, Num, Space};

#[derive(Debug, Clone, PartialEq)]
pub enum Term {
    Add,
    Product,
    Number(u64),
    Paren,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    children: Vec<Node>,
    item: Term,
}

impl Node {
    pub fn new(item: Term, children: Vec<Node>) -> Node {
        Node {
            children,
            item,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Op(char),
    Num(u64),
    Space,
}


pub fn parse_line(line: &str) -> Vec<Token> {
    line.chars().map(|ch|
        match ch {
            '*' | '+' => Op(ch),
            // Since we parse the string backwards, open and close paren are backwards
            ')' => OpenParen,
            '(' => CloseParen,
            '0'..='9' => Num(ch.to_digit(10).unwrap() as u64),
            ' ' => Space,
            _ => panic!("Invalid char parsed!")
        }
    )
        .filter(|token| token != &Space)
        .rev()
        .collect::<Vec<Token>>()
}

pub fn run_expression(expression: &Node) -> u64 {
    return match expression.item {
        Term::Add => {
            let lhs = expression.children.get(0).unwrap();
            let rhs = expression.children.get(1).unwrap();
            run_expression(lhs) + run_expression(rhs)
        }
        Term::Product => {
            let lhs = expression.children.get(0).unwrap();
            let rhs = expression.children.get(1).unwrap();
            run_expression(lhs) * run_expression(rhs)
        }
        Term::Number(n) => {
            n
        }
        Term::Paren => {
            run_expression(expression.children.get(0).unwrap())
        }
    };
}

pub fn print(tree: &Node) -> String {
    match tree.item {
        Term::Paren => {
            format!("({})",
                    print(tree.children.get(0).expect("parens need one child")))
        }
        Term::Add => {
            let lhs = print(tree.children.get(0).expect("sums need two children"));
            let rhs = print(tree.children.get(1).expect("sums need two children"));
            format!("{} + {}", lhs, rhs)
        }
        Term::Product => {
            let lhs = print(tree.children.get(0).expect("products need two children"));
            let rhs = print(tree.children.get(1).expect("products need two children"));
            format!("{} * {}", lhs, rhs)
        }
        Term::Number(n) => format!("{}", n),
    }
}
