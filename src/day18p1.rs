use crate::day18_util::Token::{CloseParen, Num, Op, OpenParen, Space};
use crate::day18_util::{Token, parse_line, run_expression, Node, Term};

pub fn solve_floor() {
    let data: Vec<Vec<Token>> = include_str!("../resources/18-1.txt")
        .lines()
        .map(|line| parse_line(line))
        .collect();

    let mut sum = 0;
    for expression in data {
        let (root_node, _) = parse_expression(expression.as_slice());
        sum += run_expression(&root_node);
    }

    println!("{}", sum);
}

fn parse_expression(tokens: &[Token]) -> (Node, &[Token]) {
    let (left_side, remaining_tokens) = parse_term(tokens).unwrap();
    match remaining_tokens.get(0) {
        Some(&Op(ch)) => {
            let item = if ch == '*' { Term::Product } else { Term::Add };
            let (right_side, remaining_tokens) = parse_expression(&remaining_tokens[1..]);
            let operation = Node::new(item, vec![left_side, right_side]);

            (operation, &remaining_tokens)
        }
        _ => {
            (left_side, remaining_tokens)
        }
    }
}

fn parse_term(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    // println!("next token is {:?}", tokens[0]);
    return match tokens[0] {
        OpenParen => {
            // Parse expressions until we find a close paren
            let (inner_node, remaining_tokens) = parse_expression(&tokens[1..]);
            let next_token = remaining_tokens.get(0);

            // println!("next token is {:?}", next_token.unwrap());
            if next_token == Some(&CloseParen) {
                let paren_node = Node::new(Term::Paren, vec![inner_node]);
                Ok((paren_node, &remaining_tokens[1..]))
            } else {
                Err("No matching close paren for open paren!".to_string())
            }
        }
        Num(n) => {
            Ok((Node::new(Term::Number(n), vec![]), &tokens[1..]))
        }
        CloseParen => {
            Err("Got close paren with no open paren!".to_string())
        }
        Op(ch) => {
            Err(format!("Got operation {} when expecting a number or an open paren", ch))
        }
        Space => {
            Err("Got space when they should be filtered out".to_string())
        }
    };
}

pub fn solve_basement() {}
