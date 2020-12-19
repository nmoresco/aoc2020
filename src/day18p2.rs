use crate::day18_util::{Node, parse_line, run_expression, Term, Token};
use crate::day18_util::Token::{CloseParen, Num, Op, OpenParen, Space};

pub fn solve_floor() {
}

pub fn solve_basement() {
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
    let (left_side, remaining_tokens) = parse_operand(tokens);
    match remaining_tokens.get(0) {
        Some(&Op('*')) => {
            println!("next token is {:?}", remaining_tokens[0]);
            let (right_side, remaining_tokens) = parse_expression(&remaining_tokens[1..]);
            let operation = Node::new(Term::Product, vec![left_side, right_side]);

            (operation, &remaining_tokens)
        }
        _ => {
            (left_side, remaining_tokens)
        }
    }
}

fn parse_operand(tokens: &[Token]) -> (Node, &[Token]) {
    let (left_side, remaining_tokens) = parse_term(tokens).unwrap();
    match remaining_tokens.get(0) {
        Some(&Op('+')) => {
            println!("next token is {:?}", remaining_tokens[0]);
            let (right_side, remaining_tokens) = parse_operand(&remaining_tokens[1..]);
            let operation = Node::new(Term::Add, vec![left_side, right_side]);

            (operation, &remaining_tokens)
        }
        _ => {
            (left_side, remaining_tokens)
        }
    }
}

fn parse_term(tokens: &[Token]) -> Result<(Node, &[Token]), String> {
    println!("next token is {:?}", tokens[0]);
    return match tokens[0] {
        OpenParen => {
            // Parse expressions until we find a close paren
            let (inner_node, remaining_tokens) = parse_expression(&tokens[1..]);
            let next_token = remaining_tokens.get(0);

            println!("next token is {:?}", next_token.unwrap());
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
