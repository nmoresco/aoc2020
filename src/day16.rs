use regex::Regex;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(PartialEq, Eq, Hash)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct Rule<'a> {
    low_range: Range,
    high_range: Range,
    label: &'a str,
}

pub fn solve_floor() {
    let (rules, _my_ticket, tickets) = parse_data();

    println!("{}", tickets.iter()
                          .flatten()
                          .filter(|&&num| !validate_field_all(num, &rules))
                          .sum::<u32>());
}

pub fn solve_basement() {
    let mut matched_rules: HashMap<usize, &Rule> = HashMap::new();

    let (rules, my_ticket, tickets) = parse_data();

    let valid_tickets: Vec<&Vec<u32>> =
        tickets.iter()
               .filter(|&ticket| ticket.iter().all(|&field| validate_field_all(field, &rules)))
               .collect::<Vec<&Vec<u32>>>();

    let mut fields_to_failed_rules: HashMap<usize, Vec<&Rule>> = HashMap::new();

    for ticket in valid_tickets {
        for (idx, field) in ticket.iter().enumerate() {
            let matched_rules: Vec<&Rule> = rules.iter().filter(|rule| !validate_field(*field, rule)).collect();
            match fields_to_failed_rules.entry(idx) {
                Entry::Occupied(mut occupied) => {
                    occupied.get_mut().extend(matched_rules);
                }
                Entry::Vacant(vacant) => {
                    vacant.insert(matched_rules);
                }
            };
        }
    }

    loop {
        let (&rule_index, remaining_rule): (&usize, &Rule) = match fields_to_failed_rules
            .iter()
            .filter(|(idx, failed_rules)| failed_rules.len() == 19 && !matched_rules.contains_key(idx))
            .map(|(idx, field)| {
                // Find the only field that didn't fail.
                (idx, rules.iter().filter(|rule| !field.contains(rule)).next().unwrap())
            }).next() {
            Some(result) => result,
            None => break
        };

        matched_rules.insert(rule_index, remaining_rule);

        for idx in 0..my_ticket.len() {
            let failed_rules = fields_to_failed_rules.get_mut(&idx).unwrap();
            if !failed_rules.contains(&remaining_rule) {
                failed_rules.push(remaining_rule);
            }
        }
    }

    println!("{}", matched_rules
        .iter()
        .filter(|(_, &rule)| rule.label.starts_with("departure"))
        .map(|(&idx, _)| my_ticket.get(idx).unwrap())
        .cloned()
        .map(|x| x as u64)
        .product::<u64>());
}

fn parse_data() -> (Vec<Rule<'static>>, Vec<u32>, Vec<Vec<u32>>) {
    let mut data = include_str!("../resources/16-1.txt")
        .split("your ticket:");

    let rules = data.next().unwrap().trim();
    let tickets = data.next().unwrap().trim();

    lazy_static! {
        static ref RULE_RE: Regex = Regex::new("([\\w ]+): (\\d+)-(\\d+) or (\\d+)-(\\d+)").unwrap();
    }

    let rules: Vec<Rule> = rules.lines().map(|line| {
        let captures = RULE_RE.captures(line).unwrap();
        Rule {
            label: captures.get(1).unwrap().as_str(),
            low_range: Range {
                min: captures.get(2).unwrap().as_str().parse().unwrap(),
                max: captures.get(3).unwrap().as_str().parse().unwrap(),
            },
            high_range: Range {
                min: captures.get(4).unwrap().as_str().parse().unwrap(),
                max: captures.get(5).unwrap().as_str().parse().unwrap(),
            },
        }
    }).collect();

    let mut tickets_itr = tickets.lines();
    let my_ticket = tickets_itr.next().unwrap()
                               .split(",")
                               .map(|val| val.parse().unwrap())
                               .collect();

    let tickets: Vec<Vec<u32>> = tickets_itr
        .filter(|line| line.contains(","))
        .map(|line| {
            line.split(",")
                .map(|val| val.parse().unwrap()).collect()
        }).collect();
    (rules, my_ticket, tickets)
}

fn validate_field_all(field: u32, rules: &Vec<Rule>) -> bool {
    rules.iter().any(|rule| {
        if (field < rule.low_range.min || field > rule.low_range.max)
            && (field < rule.high_range.min || field > rule.high_range.max) {
            return false;
        }

        return true;
    })
}


fn validate_field(field: u32, rule: &Rule) -> bool {
    if (field < rule.low_range.min || field > rule.low_range.max)
        && (field < rule.high_range.min || field > rule.high_range.max) {
        return false;
    }

    return true;
}
