use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
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
use tracing::debug;

enum Data {
    RegularNumber(i32),
    Pair(Node, Node),
}

impl Debug for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::RegularNumber(n) => write!(f, "{:?}", n),
            Data::Pair(a, b) => write!(f, "[{:?}, {:?}]", a, b)
        }
    }
}

struct NodeStruct {
    parent: Option<Node>,
    data: Data
}

impl Debug for NodeStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node<{:?}>", self.data)
    }
}

impl NodeStruct {
    fn into_node(self) -> Node {
        Rc::new(RefCell::new(self))
    }
}

type Node = Rc<RefCell<NodeStruct>>;

trait NodeBehavior {
    fn left_child(&self) -> Option<Node>;
    fn right_child(&self) -> Option<Node>;
    fn go_left(&self) -> Option<Node>;
    fn go_right(&self) -> Option<Node>;
    fn get_parent(&self) -> Option<Node>;
}

impl NodeBehavior for Node {
    fn left_child(&self) -> Option<Node> {
        match &(**self).borrow().data {
            Data::RegularNumber(_) => None,
            Data::Pair(a, _) => Some(a.clone())
        }
    }

    fn right_child(&self) -> Option<Node> {
        match &(**self).borrow().data {
            Data::RegularNumber(_) => None,
            Data::Pair(_, b) => Some(b.clone())
        }
    }

    fn go_left(&self) -> Option<Node> {
        if let Some(parent) = &(**self).borrow().parent {
            if Rc::ptr_eq(&parent.right_child().unwrap(), self) {
                parent.left_child()
            } else {
                parent.go_left()
            }
        } else {
            None
        }
    }

    fn go_right(&self) -> Option<Node> {
        if let Some(parent) = &(**self).borrow().parent {
            if Rc::ptr_eq(&parent.left_child().unwrap(), self) {
                parent.right_child()
            } else {
                parent.go_right()
            }
        } else {
            None
        }
    }

    fn get_parent(&self) -> Option<Node> {
        (**self).borrow().parent.clone()
    }
}

fn parse_number(parent: Option<Node>) -> impl Fn(&str) -> IResult<&str, Node> {
    move |i: &str| {
        digit1(i).map(|(left, n)| {
            let n = NodeStruct { parent: parent.clone(), data: Data::RegularNumber(i32::from_str(n).unwrap()) };
            (left, n.into_node())
        })
    }
}

fn parse_pair(parent: Option<Node>) -> impl Fn(&str) -> IResult<&str, Node> {
    move |i: &str| {
        tuple((
            tag("["),
            alt((parse_pair(None), parse_number(None))),
            tag(","),
            alt((parse_pair(None), parse_number(None))),
            tag("]")
        ))(i).map(|(left, (_, a, _, b, _))| {
            let n = NodeStruct { parent: parent.clone(), data: Data::Pair(a.clone(), b.clone()) }.into_node();

            (*a).borrow_mut().parent = Some(n.clone());
            (*b).borrow_mut().parent = Some(n.clone());

            (left, n)
        })
    }
}

fn parse_input(i: &str) -> IResult<&str, Vec<Node>> {
    many1(
        terminated(
            parse_pair(None),
            opt(newline)
        )
    )(i)
}

pub(crate) fn solve(input: String) -> Result<(), Report> {
    let numbers = match all_consuming(parse_input)(&input) {
        Ok((_, numbers)) => numbers,
        Err(e) => return Err(Report::msg(format!("Failed to parse input: {:?}", e)))
    };

    debug!("{:?}", numbers);
    Ok(())
}
