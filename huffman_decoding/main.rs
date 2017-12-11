extern crate common;
use common::measure_and_print;
use std::io;
use std::collections::HashMap;

// Type of the reference to the node
type Link = Option<Box<Node>>;

// Huffman tree node struct
#[derive(Debug)]
struct Node {
    letter: char,
    left: Link,
    right: Link,
}

impl Node {
    fn new_leaf(letter: char) -> Self {
        Node { letter: letter, left: None, right: None }
    }

    fn root() -> Link {
        Some(Box::new(Node::new_leaf(' ')))
    }
}

fn add_letter(root: &mut Link, letter: char, code: &str) {
    let mut p: &mut Node = root.as_mut().unwrap();
    for c in code.chars() {
        p = match {p} {
            &mut Node {left: Some(ref mut node), ..} if c == '0' => {
                node
            },
            &mut Node {left: ref mut opt @ None, ..} if c == '0' => {
                *opt = Node::root();
                opt.as_mut().unwrap()
            },
            &mut Node {right: Some(ref mut node), ..} if c == '1' => {
                node
            },
            &mut Node {right: ref mut opt @ None, ..} if c == '1' => {
                *opt = Node::root();
                opt.as_mut().unwrap()
            },
            _ => { panic!("error"); }
        }
    }
    p.letter = letter;
}

fn decode(root: &Node, code_string: &str) -> String {
    let mut message = String::new();
    let mut p: &Node = root;
    for c in code_string.chars() {
        p = match p {
            &Node {left: Some(ref node), ..} if c == '0' => { node },
            &Node {right: Some(ref node), ..} if c == '1' => { node },
            _ => { panic!("error!"); }
        };
        p = match p {
            &Node {left: None, right: None, ref letter, ..} => { 
                message.push(letter.clone());
                root
            },
            _ => p
        };
    }
    message
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn main() {
    // 1. Read input data: number of letters, code length, letter->code table, code
    let input = read_line();
    let mut iter = input.split_whitespace();
    let char_n: i32 = iter.next().unwrap().parse().unwrap();
    // let code_n: i32 = iter.next().unwrap().parse().unwrap();

    let mut table = HashMap::<char, String>::new();

    // 1. Read letter->code table, read the code message
    let mut root = Some(Box::new(Node::new_leaf(' ')));
    for _ in 0..char_n {
        let input = read_line();
        let mut iter = input.split(':');
        table.insert( iter.next().unwrap().trim().parse().unwrap()
                    , iter.next().unwrap().trim().to_string()
                    );
    }
    let code_string = read_line();

    let mut message = String::new();
    measure_and_print(||
        {
            // 2. Build the tree
            for (letter, code) in &table {
                add_letter(&mut root, *letter, code);
            }

            // 3. Decode message
            message = decode(root.as_ref().unwrap(), &code_string);
        });

    // 4. Output
    println!("{}", message);
}
