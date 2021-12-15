use std::collections::HashMap;
use color_eyre::Report;
use itertools::Itertools;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alpha1, newline, space0};
use nom::IResult;
use nom::combinator::all_consuming;
use nom::multi::many1;
use nom::sequence::tuple;
use tracing::{debug, info};

#[derive(Debug)]
struct Rule {
    input: (char, char),
    output: ((char, char), (char, char))
}

#[derive(Debug)]
struct TestCase {
    rules: Vec<Rule>,
    pairs: HashMap<(char, char), u64>,
    amounts: HashMap<char, u64>
}

impl TestCase {
    fn new(template: &str, rules: Vec<Rule>) -> TestCase {
        let mut pairs: HashMap<(char, char), u64> = HashMap::new();
        for char_pair in template.chars().tuple_windows() {
            *pairs.entry(char_pair).or_insert(0) += 1;
        }
        let mut amounts = HashMap::new();
        for ch in template.chars() {
            *amounts.entry(ch).or_insert(0) += 1
        }
        TestCase { rules, pairs, amounts }
    }

    fn step(self) -> TestCase {
        let mut new_pairs = HashMap::new();
        let mut amounts = self.amounts;
        for rule in &self.rules {
            if let Some(count) = self.pairs.get(&rule.input) {
                *new_pairs.entry(rule.output.0).or_insert(0) += count;
                *new_pairs.entry(rule.output.1).or_insert(0) += count;
                *amounts.entry(rule.output.0.1).or_insert(0) += count;
            }
        }
        TestCase { rules: self.rules, pairs: new_pairs, amounts}
    }

    fn answer(&self) -> u64 {
        self.amounts.values().max().unwrap() - self.amounts.values().min().unwrap()
    }
}

impl Rule {
    fn from_pair(input: &str, result: &str) -> Rule {
        let input = input[..2].chars().collect_tuple().unwrap();
        let (result, ) = result[..1].chars().collect_tuple().unwrap();
        Rule {
            input,
            output: ((input.0, result), (result, input.1))
        }
    }
}

fn rule_parser(i: &str) -> IResult<&str, Rule> {
    tuple((
        alpha1,
        space0,
        tag("->"),
        space0,
        alpha1,
        newline
    ))(i).map(|(left, (input, _, _, _, output, _))|
          (left, Rule::from_pair(input, output))
    )
}

fn file_parser(i: &str) -> IResult<&str, TestCase> {
    all_consuming(tuple((
        take_while(|c: char| c.is_alphabetic()),
        newline,
        newline,
        many1(
            rule_parser
        )
        )))(i)
        .map(|(left, (template, _, _, rules))| (left, TestCase::new(template, rules)))
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let mut test_case = match file_parser(&input) {
        Ok(("", test_case)) => test_case,
        // all_consuming won't return a success with anything left over
        Ok(_) => unreachable!(),
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    debug!("{:?}", test_case.pairs);

    debug!("{:?}", test_case.amounts);

    for _ in 0..10 {
        test_case = test_case.step();
        debug!("{:?}", test_case.amounts);
    }

    info!(day=14, part=1, answer=test_case.answer());

    for _ in 0..30 {
        test_case = test_case.step();
    }

    info!(day=14, part=2, answer=test_case.answer());
    Ok(())
}
