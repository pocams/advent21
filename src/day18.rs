use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;

use color_eyre::Report;
use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{all_consuming, opt};
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use tracing::info;

#[derive(Debug)]
enum Data {
    RegularNumber(i32),
    Pair(Node, Node),
}

#[derive(Debug)]
struct NodeStruct {
    depth: i32,
    data: Data
}

impl NodeStruct {
    fn into_node(self) -> Node {
        Rc::new(RefCell::new(self))
    }
}

type Node = Rc<RefCell<NodeStruct>>;

struct RegularNodeIterator {
    node: Node,
    yielded_node: bool,
}

struct PairNodeIterator {
    node: Node,
    yielded_node: bool,
    left: Box<dyn Iterator<Item=Node>>,
    right: Box<dyn Iterator<Item=Node>>,
}

fn iterate_node(node: &Node) -> Box<dyn Iterator<Item=Node>> {
    match &(*node).borrow().data {
        Data::RegularNumber(_) => {
            Box::new(RegularNodeIterator {
                node: node.clone(),
                yielded_node: false
            })
        }
        Data::Pair(a, b) => {
            Box::new(PairNodeIterator {
                node: node.clone(),
                yielded_node: false,
                left: iterate_node(a),
                right: iterate_node(b)
            })
        }
    }
}

impl Iterator for RegularNodeIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded_node {
            None
        } else {
            self.yielded_node = true;
            Some(self.node.clone())
        }
    }
}

impl Iterator for PairNodeIterator {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if self.yielded_node {
            self.left.next().or_else(|| self.right.next())
        } else {
            self.yielded_node = true;
            Some(self.node.clone())
        }
    }
}

trait NodeBehavior {
    fn get_depth(&self) -> i32;
    fn split_data(&self) -> Data;
    fn add(self, rhs: Node) -> Node;
    fn has_value(&self) -> bool;
    fn get_value(&self) -> i32;
    fn set_value(&self, value: i32);
    fn deep_copy(&self) -> Node;
}

impl NodeBehavior for Node {
    fn get_depth(&self) -> i32 {
        (**self).borrow().depth
    }

    fn split_data(&self) -> Data {
        let value = self.get_value();
        Data::Pair(
            NodeStruct { depth: self.get_depth() + 1, data: Data::RegularNumber(value / 2) }.into_node(),
            NodeStruct { depth: self.get_depth() + 1, data: Data::RegularNumber(value / 2 + value % 2) }.into_node(),
        )
    }

    // Can't impl Add because Node is just a type alias
    fn add(self, rhs: Node) -> Node {
        for n in iterate_node(&self) {
            n.borrow_mut().depth += 1;
        }
        for n in iterate_node(&rhs) {
            n.borrow_mut().depth += 1;
        }

        NodeStruct { depth: 0, data: Data::Pair(self, rhs) }.into_node()
    }

    fn deep_copy(&self) -> Node {
        match &(**self).borrow().data {
            Data::RegularNumber(n) => NodeStruct { depth: self.get_depth(), data: Data::RegularNumber(*n) }.into_node(),
            Data::Pair(a, b) => NodeStruct { depth: self.get_depth(), data: Data::Pair(a.deep_copy(), b.deep_copy()) }.into_node()
        }
    }

    // FIXME: these restrictions should deeeeeeefinitely be enforced at compile time
    fn has_value(&self) -> bool {
        matches!(&(**self).borrow().data, Data::RegularNumber(_))
    }

    fn get_value(&self) -> i32 {
        match &(**self).borrow().data {
            Data::RegularNumber(n) => *n,
            _ => panic!("get_value() called on non-regular-number node")
        }
    }

    fn set_value(&self, value: i32) {
        let n = &mut(**self).borrow_mut();
        match n.data {
            Data::RegularNumber(_) => n.data = Data::RegularNumber(value),
            _ => panic!("set_value() called on non-regular-number node")
        }
    }
}

fn parse_number(depth: i32) -> impl Fn(&str) -> IResult<&str, Node> {
    move |i: &str| {
        digit1(i).map(|(left, n)| {
            let n = NodeStruct { depth, data: Data::RegularNumber(i32::from_str(n).unwrap()) };
            (left, n.into_node())
        })
    }
}

fn parse_pair(depth: i32) -> impl Fn(&str) -> IResult<&str, Node> {
    move |i: &str| {
        tuple((
            tag("["),
            alt((parse_pair(depth+1), parse_number(depth+1))),
            tag(","),
            alt((parse_pair(depth+1), parse_number(depth+1))),
            tag("]")
        ))(i).map(|(left, (_, a, _, b, _))| {
            let n = NodeStruct { depth, data: Data::Pair(a.clone(), b.clone()) }.into_node();

            (left, n)
        })
    }
}

fn parse_input(i: &str) -> IResult<&str, Vec<Node>> {
    many1(
        terminated(
            parse_pair(0),
            opt(newline)
        )
    )(i)
}

fn print_node(node: &Node) {
    match &(**node).borrow().data {
        Data::RegularNumber(n) => print!("{}", n),
        Data::Pair(a, b) => {
            print!("[");
            print_node(a);
            print!(",");
            print_node(b);
            print!("]");
        }
    }
}

fn try_explode(node: &Node) -> bool {
    // This is, eh, not a very satisfying implementation
    let mut previous_number: Option<Node> = None;
    let mut to_add: Option<i32> = None;

    let mut iter = iterate_node(node);
    for n in &mut iter {
        if n.has_value() {
            previous_number = Some(n);
        } else if n.get_depth() == 4 {
            // from AoC site:
            // Exploding pairs will always consist of two regular numbers.
            let mut explode_parent = n.borrow_mut();
            match &explode_parent.data {
                Data::Pair(explode_left, explode_right) => {
                    if let Some(prev) = &previous_number {
                        prev.set_value(prev.get_value() + explode_left.get_value())
                    }
                    to_add = Some(explode_right.get_value());
                }
                // Unreachable because has_value() returned false above
                Data::RegularNumber(_) => unreachable!()
            }
            explode_parent.data = Data::RegularNumber(0);
            // Skip over the now-exploded contents of this node
            iter.next();
            iter.next();
            break
        }
    }

    if let Some(add) = to_add {
        for n in &mut iter {
            if n.has_value() {
                // debug!("adding {} to {:?}", add, n);
                n.set_value(n.get_value() + add);
                break
            }
        }
    }

    // If to_add is Some, we exploded something; otherwise we didn't
    to_add.is_some()
}

fn try_split(node: &Node) -> bool {
    let new_data: Option<Data>;

    match &node.borrow().data {
        Data::Pair(a, b) => {
            return try_split(a) || try_split(b)
        }
        Data::RegularNumber(i) => {
            if *i >= 10 {
                new_data = Some(node.split_data());
            } else {
                return false
            }
        }
    }

    // Now that our first borrow has expired, we can do a new mutable borrow to update the node
    node.borrow_mut().data = new_data.unwrap();
    true
}

fn reduce(node: &Node) {
    // print!("reduce:  "); print_node(node); println!();
    loop {
        if try_explode(node) {
            continue
        }
        if try_split(node) {
            continue
        }
        break;
    }
}

fn magnitude(node: &Node) -> i32 {
    match &node.borrow().data {
        Data::Pair(a, b) => {
            magnitude(a) * 3 + magnitude(b) * 2
        }
        Data::RegularNumber(i) => {
            *i
        }
    }
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let numbers = match all_consuming(parse_input)(&input) {
        Ok((_, numbers)) => numbers,
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    // We're going to mess up the nodes in part 1
    let p2_numbers: Vec<_> = numbers.iter().map(|n| n.deep_copy()).collect();

    let result = numbers.into_iter().reduce(|a, b| {
        // print!("  ");
        // print_node(&a);
        // println!();
        // print!("+ ");
        // print_node(&b);
        // println!();
        let n = a.add(b);
        reduce(&n);
        // print!("= ");
        // print_node(&n);
        // println!("\n");
        n
    } ).unwrap();

    print_node(&result);
    println!();

    info!(day=18, part=1, answer=magnitude(&result));

    let mut highest_magnitude = 0;
    for i in 0..p2_numbers.len() {
        for j in 0..p2_numbers.len() {
            if i == j { continue }
            let total = p2_numbers[i].deep_copy().add(p2_numbers[j].deep_copy());
            reduce(&total);
            let mag = magnitude(&total);
            if mag > highest_magnitude {
                /*
                println!("new highest: {}", mag);
                print_node(&p2_numbers[i]);
                println!();
                print_node(&p2_numbers[j]);
                println!();
                print_node(&total);
                println!();
                println!();
                */
                highest_magnitude = mag
            }
        }
    }

    info!(day=18, part=2, answer=highest_magnitude);
    Ok(())
}
